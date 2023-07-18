use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;
use std::fmt::Debug;
use std::marker::PhantomData;

use indexmap::{IndexMap, IndexSet};
use serde::__private::de::{Content, ContentRefDeserializer};
use serde_json::Value;
use std::ops::Deref;
use std::sync::Arc;

pub trait Prepare {
    fn prepare(self) -> Self;
}

pub trait DiffContext {
    fn removing(&self) -> Self;
    fn switch_flow(&self) -> Self;

    fn is_direct_flow(&self) -> bool;

    fn add_visited_reference_source(&self, reference: &str) -> Self;
    fn check_visited_reference_source(&self, reference: &str) -> usize;
    fn add_visited_reference_target(&self, reference: &str) -> Self;
    fn check_visited_reference_target(&self, reference: &str) -> usize;
}

pub trait ComponentContainer<T> {
    fn deref_source(&self, reference: &str) -> Option<&T>;
    fn deref_target(&self, reference: &str) -> Option<&T>;
}

pub trait DiffCache<O> {
    fn get_diff(&self, reference: &str) -> Option<Arc<DiffResult<O>>>;
    fn set_diff(&self, reference: &str, component: Arc<DiffResult<O>>);
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "t", content = "v")]
pub enum DiffResult<T> {
    #[serde(rename = "n")]
    None,
    #[serde(rename = "=")]
    Same(T),
    #[serde(rename = "+")]
    Added(T),
    #[serde(rename = "~")]
    Updated(T, Option<Box<T>>),
    #[serde(rename = "-")]
    Removed(T),
}

impl<T> DiffResult<T> {
    pub fn is_none(&self) -> bool {
        matches!(self, DiffResult::None)
    }

    pub fn is_same(&self) -> bool {
        matches!(self, DiffResult::Same(_))
    }

    pub fn is_added(&self) -> bool {
        matches!(self, DiffResult::Added(_))
    }

    pub fn is_updated(&self) -> bool {
        matches!(self, DiffResult::Updated(_, _))
    }

    pub fn is_upserted(&self) -> bool {
        self.is_added() || self.is_updated()
    }

    pub fn is_removed(&self) -> bool {
        matches!(self, DiffResult::Removed(_))
    }

    pub fn is_same_or_none(&self) -> bool {
        self.is_same() || self.is_none()
    }

    pub fn exists(&self) -> bool {
        !(self.is_none() || self.is_removed())
    }

    pub fn new<C: DiffContext>(mut value: DiffResult<T>, context: &C) -> Self {
        if !context.is_direct_flow() {
            value = match value {
                DiffResult::Added(v) => DiffResult::Removed(v),
                DiffResult::Removed(v) => DiffResult::Added(v),

                DiffResult::Updated(new, Some(old)) => {
                    DiffResult::Updated(*old, Some(Box::new(new)))
                }
                result => result,
            }
        }

        value
    }

    pub fn get(&self) -> Option<&T> {
        match self {
            DiffResult::None => None,

            DiffResult::Same(v) => Some(v),
            DiffResult::Added(v) => Some(v),
            DiffResult::Removed(v) => Some(v),
            DiffResult::Updated(v, _) => Some(v),
        }
    }

    pub fn take(self) -> Option<T> {
        match self {
            DiffResult::None => None,

            DiffResult::Same(v) => Some(v),
            DiffResult::Added(v) => Some(v),
            DiffResult::Removed(v) => Some(v),
            DiffResult::Updated(v, _) => Some(v),
        }
    }

    pub fn as_ref(&self) -> DiffResult<&T> {
        match &self {
            DiffResult::None => DiffResult::None,
            DiffResult::Same(v) => DiffResult::Same(v),
            DiffResult::Added(v) => DiffResult::Added(v),
            DiffResult::Removed(v) => DiffResult::Removed(v),

            DiffResult::Updated(new, old) => DiffResult::Updated(
                new,
                old.as_ref().map(|old| Box::new(&**old)),
            ),
        }
    }
}

// Marker trait
pub trait Referencable {}

impl Referencable for Value {}

pub trait Diff<With, Output, Context: DiffContext> {
    fn diff(
        &self,
        new: Option<&With>,
        context: &Context,
    ) -> DiffResult<Output>;
}

pub trait Empty {
    fn is_empty(&self) -> bool;
}

pub trait Keyed<C> {
    fn key(&self, c: C) -> String;
}

impl<T, O, C: DiffContext> Diff<T, O, C> for Option<T>
where
    T: Diff<T, O, C> + Debug,
{
    fn diff(&self, new: Option<&T>, context: &C) -> DiffResult<O> {
        match (self, new) {
            (None, None) => DiffResult::new(DiffResult::None, context),
            (Some(v1), Some(v2)) => v1.diff(Some(v2), context),
            (Some(v1), None) => v1.diff(None, context),
            (None, Some(v2)) => {
                if context.is_direct_flow() {
                    v2.diff(None, &context.switch_flow())
                } else {
                    v2.diff(None, context)
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(Box<R>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "t", content = "v")]
pub enum EitherDiff<LD, RD> {
    #[serde(rename = "l")]
    Left(DiffResult<LD>),
    #[serde(rename = "r")]
    Right(Box<DiffResult<RD>>),

    #[serde(rename = "tr")]
    ToRight(Box<DiffResult<RD>>),
    #[serde(rename = "tl")]
    ToLeft(Box<DiffResult<LD>>),
}

impl<LD, RD> Empty for EitherDiff<LD, RD> {
    fn is_empty(&self) -> bool {
        match self {
            EitherDiff::Left(l) => l.is_same_or_none(),
            EitherDiff::Right(r) => r.is_same_or_none(),
            _ => false,
        }
    }
}

impl<L, R, LD, RD, C: DiffContext> Diff<Either<L, R>, EitherDiff<LD, RD>, C>
    for Either<L, R>
where
    L: Diff<L, LD, C>,
    R: Diff<R, RD, C>,
{
    fn diff(
        &self,
        new: Option<&Either<L, R>>,
        context: &C,
    ) -> DiffResult<EitherDiff<LD, RD>> {
        let diff = match new {
            None => DiffResult::Removed(match self {
                Either::Left(l) => {
                    EitherDiff::Left(l.diff(None, &context.removing()))
                }
                Either::Right(r) => EitherDiff::Right(Box::new(
                    r.diff(None, &context.removing()),
                )),
            }),
            Some(value) => {
                let diff = match value {
                    Either::Left(vl) => match self {
                        Either::Left(l) => {
                            EitherDiff::Left(l.diff(Option::from(vl), context))
                        }
                        Either::Right(_) => EitherDiff::ToLeft(Box::new(
                            vl.diff(None, context),
                        )),
                    },
                    Either::Right(vr) => match self {
                        Either::Left(_) => EitherDiff::ToRight(Box::new(
                            vr.diff(None, context),
                        )),
                        Either::Right(r) => EitherDiff::Right(Box::new(
                            r.diff(Some(vr), context),
                        )),
                    },
                };

                if diff.is_empty() {
                    DiffResult::Same(diff)
                } else {
                    DiffResult::Updated(diff, None)
                }
            }
        };
        DiffResult::new(diff, context)
    }
}

pub trait ReferenceDescriptor {
    fn reference(&self) -> &str;
}

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum MayBeRefCore<T, R: ReferenceDescriptor> {
    Ref(R),
    Value(T),
}

impl<'de, T, R: ReferenceDescriptor> Deserialize<'de> for MayBeRefCore<T, R>
where
    T: Deserialize<'de>,
    R: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let content = match <Content as Deserialize>::deserialize(deserializer)
        {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        };

        if let Ok(ok) = Result::map(
            <R as Deserialize>::deserialize(
                ContentRefDeserializer::<D::Error>::new(&content),
            ),
            MayBeRefCore::Ref,
        ) {
            return Ok(ok);
        }

        let mut track = serde_path_to_error::Track::new();
        let de = serde_path_to_error::Deserializer::new(
            ContentRefDeserializer::<D::Error>::new(&content),
            &mut track,
        );

        match <T as Deserialize>::deserialize(de) {
            Ok(t) => Ok(MayBeRefCore::Value(t)),
            Err(err) => {
                eprintln!("Err path: {} --- {}", track.path(), &err);
                Err(err)
            }
        }
    }
}

impl<T, R: ReferenceDescriptor> MayBeRefCore<T, R> {
    pub fn is_ref(&self) -> bool {
        matches!(self, MayBeRefCore::Ref(_))
    }

    pub fn reference(&self) -> Option<&str> {
        match self {
            MayBeRefCore::Ref(rd) => Some(rd.reference()),
            _ => None,
        }
    }

    pub fn value(&self) -> Option<&T> {
        match self {
            MayBeRefCore::Value(v) => Some(v),
            _ => None,
        }
    }

    pub fn value_mut(&mut self) -> Option<&mut T> {
        match self {
            MayBeRefCore::Value(v) => Some(v),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase", tag = "t", content = "v")]
pub enum MayBeRefCoreDiff<T: Referencable, R: ReferenceDescriptor> {
    #[serde(rename = "r")]
    Ref(R),
    #[serde(rename = "v")]
    Value(Arc<DiffResult<T>>),
}

impl<T, R> Keyed<usize> for MayBeRefCore<T, R>
where
    T: Keyed<usize>,
    R: ReferenceDescriptor,
{
    fn key(&self, c: usize) -> String {
        match self {
            MayBeRefCore::Value(value) => value.key(c),
            MayBeRefCore::Ref(value) => value.reference().to_owned(),
        }
    }
}

impl<T, O, R, C: DiffContext>
    Diff<MayBeRefCore<T, R>, MayBeRefCoreDiff<O, R>, C> for MayBeRefCore<T, R>
where
    T: Diff<T, O, C> + Clone + Debug + 'static,
    O: Referencable + Clone + Debug + 'static,
    R: ReferenceDescriptor + Clone + Debug + 'static,
    C: DiffContext + ComponentContainer<T> + DiffCache<O> + ToOwned<Owned = C>,
{
    fn diff(
        &self,
        new: Option<&MayBeRefCore<T, R>>,
        context: &C,
    ) -> DiffResult<MayBeRefCoreDiff<O, R>> {
        let diff = match new {
            None => DiffResult::Removed(match self {
                MayBeRefCore::Ref(value) => {
                    MayBeRefCoreDiff::Ref(value.clone())
                }
                MayBeRefCore::Value(value) => MayBeRefCoreDiff::Value(
                    Arc::new(value.diff(None, context)),
                ),
            }),
            Some(value) => {
                let cached_diff = if let (
                    MayBeRefCore::Ref(old_ref),
                    MayBeRefCore::Ref(new_ref),
                ) = (self, value)
                {
                    if old_ref.reference() == new_ref.reference() {
                        context.get_diff(old_ref.reference())
                    } else {
                        None
                    }
                } else {
                    None
                };

                let diff = if let Some(diff) = &cached_diff {
                    Arc::clone(diff)
                } else {
                    let context =
                        if let MayBeRefCore::Ref(old_ref) = self {
                            Cow::Owned(context.add_visited_reference_source(
                                old_ref.reference(),
                            ))
                        } else {
                            Cow::Borrowed(context)
                        };
                    let context =
                        if let MayBeRefCore::Ref(new_ref) = value {
                            Cow::Owned(context.add_visited_reference_target(
                                new_ref.reference(),
                            ))
                        } else {
                            context
                        };

                    let (old_value, new_value) = match (self, value) {
                        (
                            MayBeRefCore::Ref(old_ref),
                            MayBeRefCore::Ref(new_ref),
                        ) => {
                            let old_reference = old_ref.reference();
                            let new_reference = new_ref.reference();

                            let old_visited_count = context
                                .check_visited_reference_source(old_reference);
                            let new_visited_count = context
                                .check_visited_reference_target(new_reference);

                            if old_visited_count > 1 && new_visited_count > 1 {
                                return DiffResult::None;
                            }

                            let source = context.deref_source(old_reference);
                            let target = context.deref_target(new_reference);

                            (source, target)
                        }
                        (
                            MayBeRefCore::Value(old_value),
                            MayBeRefCore::Value(new_value),
                        ) => (Some(old_value), Some(new_value)),

                        (
                            MayBeRefCore::Value(old_value),
                            MayBeRefCore::Ref(new_ref),
                        ) => {
                            let target =
                                context.deref_target(new_ref.reference());
                            (Some(old_value), target)
                        }

                        (
                            MayBeRefCore::Ref(old_ref),
                            MayBeRefCore::Value(new_value),
                        ) => {
                            let source =
                                context.deref_source(old_ref.reference());
                            (source, Some(new_value))
                        }
                    };

                    let old_value = if let Some(value) = old_value {
                        value
                    } else {
                        let diff = None.diff(new_value, &*context);
                        return if diff.is_none() {
                            DiffResult::new(DiffResult::None, &*context)
                        } else {
                            DiffResult::new(
                                DiffResult::Added(MayBeRefCoreDiff::Value(
                                    Arc::new(diff),
                                )),
                                &*context,
                            )
                        };
                    };

                    let new_value = if let Some(value) = new_value {
                        value
                    } else {
                        // removed schema
                        return self.diff(None, &*context);
                    };

                    Arc::new(old_value.diff(Some(new_value), &*context))
                };

                match (self, value) {
                    (
                        MayBeRefCore::Ref(old_ref),
                        MayBeRefCore::Ref(new_ref),
                    ) if old_ref.reference() == new_ref.reference() => {
                        let result = match &*diff {
                            DiffResult::None => DiffResult::None,
                            DiffResult::Same(_) => DiffResult::Same(
                                MayBeRefCoreDiff::Ref(old_ref.clone()),
                            ),
                            DiffResult::Added(_) => DiffResult::Added(
                                MayBeRefCoreDiff::Ref(old_ref.clone()),
                            ),
                            DiffResult::Removed(_) => DiffResult::Removed(
                                MayBeRefCoreDiff::Ref(old_ref.clone()),
                            ),
                            DiffResult::Updated(_, _) => DiffResult::Updated(
                                MayBeRefCoreDiff::Ref(old_ref.clone()),
                                None,
                            ),
                        };

                        if cached_diff.is_none() {
                            context.set_diff(old_ref.reference(), diff);
                        }

                        result
                    }
                    // Do not track `ref -> value` and `value -> ref`
                    // since they are not representable changes for us,
                    // we are trying to track only changes that on UI look different
                    (_, _) => {
                        if diff.is_same_or_none() {
                            DiffResult::Same(MayBeRefCoreDiff::Value(diff))
                        } else {
                            DiffResult::Updated(
                                MayBeRefCoreDiff::Value(diff),
                                None,
                            )
                        }
                    }
                }
            }
        };

        DiffResult::new(diff, context)
    }
}

impl<V> Keyed<usize> for IndexMap<String, V> {
    fn key(&self, idx: usize) -> String {
        idx.to_string()
    }
}

pub trait PathResolver {
    /// Object implementing this trait can determine that two different
    /// keys from src and tgt represents the same object and thus must be mapped
    fn new<'a, T>(k1: T, k2: T) -> Self
    where
        T: Iterator<Item = &'a String>;

    /// k1 is the key from src which must be mapped to tgt (k2) key
    /// or to be returned not changed
    fn k1tok2(&self, k1: &String) -> String;

    /// k2 is the key from tgt which must be mapped to src (k1) key
    /// or to be returned not changed
    fn k2tok1(&self, k2: &String) -> String;
}

#[derive(Debug, Clone)]
pub struct DefaultMapPathResolver;

impl PathResolver for DefaultMapPathResolver {
    fn new<'a, T>(_k1: T, _k2: T) -> Self
    where
        T: Iterator<Item = &'a String>,
    {
        Self
    }

    fn k1tok2(&self, k1: &String) -> String {
        k1.to_owned()
    }

    fn k2tok1(&self, k2: &String) -> String {
        k2.to_owned()
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MapDiff<V, R = DefaultMapPathResolver>(
    pub(crate) IndexMap<String, DiffResult<V>>,
    #[serde(skip)] PhantomData<R>,
);

impl<V, R> Deref for MapDiff<V, R> {
    type Target = IndexMap<String, DiffResult<V>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<V, O, C, R> Diff<IndexMap<String, V>, MapDiff<O, R>, C>
    for IndexMap<String, V>
where
    V: Diff<V, O, C> + Clone + Debug,
    R: PathResolver,
    C: DiffContext,
    O: Debug,
{
    fn diff(
        &self,
        new: Option<&IndexMap<String, V>>,
        context: &C,
    ) -> DiffResult<MapDiff<O, R>> {
        let diff = match new {
            None => DiffResult::Removed(MapDiff(
                self.iter()
                    .map(|(key, value)| {
                        (key.to_owned(), value.diff(None, &context.removing()))
                    })
                    .collect(),
                PhantomData,
            )),
            Some(other) => {
                let resolver = R::new(self.keys(), other.keys());

                let mut result: IndexMap<String, DiffResult<O>> =
                    Default::default();

                for (k1, v1) in self.iter() {
                    let k2 = resolver.k1tok2(k1);

                    if let Some(v2) = other.get(&k2) {
                        result.insert(k2, v1.diff(Some(v2), context));
                    } else {
                        result.insert(k2, v1.diff(None, context));
                    }
                }

                result.extend(other.iter().filter_map(|(k2, value)| {
                    let k1 = resolver.k2tok1(k2);
                    if self.contains_key(&k1) {
                        None
                    } else {
                        Some((k1, None.diff(Some(value), context)))
                    }
                }));

                let is_same =
                    result.iter().all(|(_key, value)| value.is_same_or_none());

                let diff = MapDiff(result, PhantomData);

                if is_same {
                    DiffResult::Same(diff)
                } else {
                    DiffResult::Updated(diff, None)
                }
            }
        };
        DiffResult::new(diff, context)
    }
}

pub trait VecDiffTransformer<T> {
    fn transform(collection: T) -> T;
}

#[derive(Default, Debug, Clone)]
pub struct DefaultVecDiffTransformer;

impl<T> VecDiffTransformer<T> for DefaultVecDiffTransformer {
    fn transform(collection: T) -> T {
        collection
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct VecDiff<T, S = DefaultVecDiffTransformer>(
    pub Vec<DiffResult<T>>,
    #[serde(skip)] PhantomData<S>,
);

impl<T, S> Deref for VecDiff<T, S> {
    type Target = Vec<DiffResult<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, O, C: DiffContext, S> Diff<Vec<T>, VecDiff<O, S>, C> for Vec<T>
where
    T: Diff<T, O, C> + Keyed<usize> + Debug,
    S: VecDiffTransformer<Vec<DiffResult<O>>>,
    O: Debug,
{
    fn diff(
        &self,
        new: Option<&Vec<T>>,
        context: &C,
    ) -> DiffResult<VecDiff<O, S>> {
        let diff = match new {
            None => DiffResult::Removed(VecDiff(
                self.iter().map(|x| x.diff(None, context)).collect(),
                PhantomData,
            )),
            Some(value) => {
                let o: IndexMap<_, _> = self
                    .iter()
                    .enumerate()
                    .map(|(idx, v)| (v.key(idx), v))
                    .collect();

                let n: IndexMap<_, _> = value
                    .iter()
                    .enumerate()
                    .map(|(idx, v)| (v.key(idx), v))
                    .collect();

                let o_keys: IndexSet<_> = o.keys().collect();
                let n_keys: IndexSet<_> = n.keys().collect();

                let new_keys: IndexSet<_> =
                    n_keys.difference(&o_keys).collect();
                let removed_keys: IndexSet<_> =
                    o_keys.difference(&n_keys).collect();
                let updated_keys: IndexSet<_> =
                    o_keys.intersection(&n_keys).collect();

                let added: Vec<_> = new_keys
                    .into_iter()
                    .map(|key| {
                        let new = n.get(*key).expect("key must present");
                        None.diff(Some(*new), context)
                    })
                    .collect();

                let removed: Vec<_> = removed_keys
                    .into_iter()
                    .map(|key| {
                        let old = o.get(*key).expect("key must present");
                        old.diff(None, context)
                    })
                    .collect();

                let updated: Vec<_> = updated_keys
                    .into_iter()
                    .map(|key| {
                        let old = o.get(*key).expect("key must present");
                        let new = n.get(*key).expect("key must present");
                        old.diff(Some(*new), context)
                    })
                    .collect();

                let values: Vec<_> = added
                    .into_iter()
                    .chain(updated.into_iter())
                    .chain(removed.into_iter())
                    .collect();

                let is_same =
                    values.iter().all(|value| value.is_same_or_none());

                let values = S::transform(values);

                let diff = VecDiff(values, PhantomData);

                if is_same {
                    DiffResult::Same(diff)
                } else {
                    DiffResult::Updated(diff, None)
                }
            }
        };
        DiffResult::new(diff, context)
    }
}

impl<C: DiffContext> Diff<Value, Value, C> for Value {
    fn diff(&self, new: Option<&Value>, context: &C) -> DiffResult<Value> {
        let diff = match new {
            None => DiffResult::Removed(self.clone()),
            Some(value) => {
                if self == value {
                    DiffResult::Same(value.clone())
                } else {
                    DiffResult::Updated(
                        value.clone(),
                        Some(Box::new(self.clone())),
                    )
                }
            }
        };
        DiffResult::new(diff, context)
    }
}

impl Keyed<usize> for Value {
    fn key(&self, idx: usize) -> String {
        match self {
            Value::Null => "null".to_owned(),
            Value::Bool(value) => format!("[bool]:{value}"),
            Value::Number(value) => format!("[number]:{value}"),
            Value::String(value) => format!("[string]:{value}"),
            // TODO: add type to Value Array and Object keys
            Value::Array(_value) => format!("[array]: {idx}"),
            Value::Object(_value) => format!("[object]: {idx}"),
        }
    }
}

impl<'a> Keyed<usize> for &'a str {
    fn key(&self, _: usize) -> String {
        self.to_string()
    }
}

impl Keyed<usize> for String {
    fn key(&self, _: usize) -> String {
        self.clone()
    }
}

impl Keyed<usize> for (String, String) {
    fn key(&self, _: usize) -> String {
        self.0.clone()
    }
}

macro_rules! impl_keyed_diff {
    ($typ:ty) => {
        impl Keyed<usize> for $typ {
            fn key(&self, _: usize) -> String {
                self.to_string()
            }
        }

        impl<C: DiffContext> Diff<$typ, $typ, C> for $typ {
            fn diff(
                &self,
                new: Option<&$typ>,
                context: &C,
            ) -> DiffResult<$typ> {
                let diff = match new {
                    None => DiffResult::Removed(*self),
                    Some(value) => {
                        if self == value {
                            DiffResult::Same(*value)
                        } else {
                            DiffResult::Updated(*value, Some(Box::new(*self)))
                        }
                    }
                };
                DiffResult::new(diff, context)
            }
        }
    };
}

impl_keyed_diff!(u32);
impl_keyed_diff!(u64);
impl_keyed_diff!(i32);
impl_keyed_diff!(i64);
impl_keyed_diff!(f32);
impl_keyed_diff!(f64);
impl_keyed_diff!(bool);
impl_keyed_diff!(usize);

impl<C: DiffContext> Diff<Self, String, C> for String {
    fn diff(&self, new: Option<&String>, context: &C) -> DiffResult<String> {
        let diff = match new {
            None => DiffResult::Removed(self.clone()),
            Some(value) => {
                if self == value {
                    DiffResult::Same(value.to_string())
                } else {
                    DiffResult::Updated(
                        value.to_string(),
                        Some(Box::new(self.to_string())),
                    )
                }
            }
        };
        DiffResult::new(diff, context)
    }
}
