import diff from "fast-diff";
import { ReactElement } from "react";

export enum PrimaryOperation {
  Same = "s",
  Added = "a",
  Updated = "u",
  Removed = "r",
}

export enum DiffResultIs {
  None = "n",
  Same = "=",
  Added = "+",
  Updated = "~",
  Removed = "-",
}

export enum MayBeRefDiffIs {
  Value = "v",
  Ref = "r",
}

export enum EitherDiffIs {
  Left = "l",
  Right = "r",
  ToLeft = "tl",
  ToRight = "tr",
}

export type DiffResultValue<T> = {
  t: DiffResultIs.Added | DiffResultIs.Removed | DiffResultIs.Same;
  v: T;
};

export type DiffResultNull = {
  t: DiffResultIs.None;
  v: null;
};

export type DiffResultUpdated<T> = {
  t: DiffResultIs.Updated;
  v: [T] | [T, T];
};

export type DiffResult<T> =
  | DiffResultValue<T>
  | DiffResultUpdated<T>
  | DiffResultNull;

export function textDiffRaw(oldValue: string, newValue: string): any {
  let result = diff(oldValue, newValue);

  return (
    <>
      {result.map(([diff, value], idx) => {
        let style = undefined;
        if (diff === -1) {
          style = { color: "red", textDecoration: "line-through" };
        } else if (diff === 1) {
          style = { color: "green" };
        }
        return (
          <span key={idx} style={style}>
            {value}
          </span>
        );
      })}
    </>
  );
}

export function textDiff(
  field: DiffResult<string> | undefined,
  primary: PrimaryOperation
): ReactElement | undefined {
  let [oldValue, same, newValue] = field
    ? valueOfChange(field, primary)
    : [null, null, null];

  if (same) {
    return <span>{same}</span>;
  }

  if (oldValue === null) {
    return <span>{newValue}</span>;
  }
  if (newValue === null) {
    return <span>{oldValue}</span>;
  }

  return textDiffRaw(oldValue, newValue);
}

export function apply<T>(
  parent: PrimaryOperation,
  current: DiffResult<T> | undefined
): PrimaryOperation {
  if (current === undefined) {
    return PrimaryOperation.Same;
  }

  if (
    parent === PrimaryOperation.Removed ||
    current.t === DiffResultIs.Removed
  ) {
    return PrimaryOperation.Removed;
  }

  if (parent === PrimaryOperation.Added) {
    return PrimaryOperation.Added;
  }

  if (parent === PrimaryOperation.Same || parent === PrimaryOperation.Updated) {
    switch (current.t) {
      case DiffResultIs.Same:
        return PrimaryOperation.Same;
      case DiffResultIs.Added:
        return PrimaryOperation.Added;
      case DiffResultIs.Updated:
        return PrimaryOperation.Updated;
    }
  }

  return PrimaryOperation.Same;
}

export function valueOfApplied<T>(
  diff: DiffResult<T>,
  primary: PrimaryOperation
): [T | null, PrimaryOperation] {
  switch (primary) {
    case PrimaryOperation.Added:
      if (diff.t === DiffResultIs.Removed) {
        return [null, primary];
      }
      break;
    case PrimaryOperation.Removed:
      if (diff.t === DiffResultIs.Added) {
        return [null, primary];
      }
      break;
  }

  return [valueOf(diff), apply(primary, diff)];
}

export function valueOf<T>(diff: DiffResult<T>): T | null {
  switch (diff.t) {
    case DiffResultIs.Same:
    case DiffResultIs.Added:
    case DiffResultIs.Removed:
      return diff.v;
    case DiffResultIs.Updated:
      return diff.v[0];
    case DiffResultIs.None:
      return null;
  }
}

export function selectPrimary<T, R>(
  diff: DiffResult<T>,
  primary: PrimaryOperation,

  none: R,
  sameOld: R,
  sameNew: R,

  added: R,
  updated: R,
  removed: R
): R {
  switch (primary) {
    case PrimaryOperation.Same:
    case PrimaryOperation.Updated:
      switch (diff.t) {
        case DiffResultIs.Same:
          return sameOld;
        case DiffResultIs.Added:
          return added;
        case DiffResultIs.Removed:
          return removed;
        case DiffResultIs.Updated:
          return updated;
        default:
          return none;
      }

    case PrimaryOperation.Added:
      switch (diff.t) {
        case DiffResultIs.Same:
        case DiffResultIs.Updated:
        case DiffResultIs.Added:
          return sameNew;
        default:
          return none;
      }

    case PrimaryOperation.Removed:
      switch (diff.t) {
        case DiffResultIs.Same:
        case DiffResultIs.Updated:
        case DiffResultIs.Removed:
          return sameOld;
        default:
          return none;
      }
  }
}

export function getBgColor<T>(
  diff: DiffResult<T> | undefined,
  primary: PrimaryOperation,
  skipUpdated: boolean = false
): string | undefined {
  if (!diff) {
    return undefined;
  }

  let color = selectPrimary(
    diff,
    primary,
    "transparent",
    "transparent",
    "transparent",
    "green.100",
    "rgba(255, 200, 51, 0.3)",
    "red.100"
  );

  if (color === "rgba(255, 200, 51, 0.3)" && skipUpdated) {
    return "transparent";
  }

  return color;
}

function _oldNewValues<T>(diff: DiffResult<T>): [T | null, T | null] {
  switch (diff.t) {
    case DiffResultIs.Same:
      return [diff.v, diff.v];
    case DiffResultIs.Added:
      return [null, diff.v];
    case DiffResultIs.Removed:
      return [diff.v, null];
    case DiffResultIs.Updated:
      return [diff.v[1] || null, diff.v[0]];
    case DiffResultIs.None:
      return [null, null];
  }
}

export const selectValue = <T, R>(
  diff: DiffResult<T> | undefined,
  primary: PrimaryOperation,
  same: (value: T) => R,
  added: (value: T) => R,
  updated: (value: T, oldValue?: T) => R,
  removed: (value: T) => R
): R | null => {
  if (!diff) {
    return null;
  }

  let [oldValue, newValue] = _oldNewValues(diff);

  let getter = selectPrimary(
    diff,
    primary,
    () => null,
    () => oldValue && same(oldValue),
    () => newValue && same(newValue),
    () => newValue && added(newValue),
    () => oldValue && newValue && updated(newValue, oldValue),
    () => oldValue && removed(oldValue)
  );

  return getter();
};

export function valueOfChange<T>(
  diff: DiffResult<T>,
  primary: PrimaryOperation
): [T | null, T | null, T | null] {
  const [oldValue, newValue] = _oldNewValues(diff);

  const value = selectPrimary(
    diff,
    primary,
    [null, null, null],
    [null, oldValue, null],
    [null, newValue, null],
    [null, null, newValue],
    [oldValue, null, newValue],
    [oldValue, null, null]
  );

  return value as [T | null, T | null, T | null];
}

export interface EitherDiffLeft<L> {
  t: EitherDiffIs.Left;
  v: DiffResult<L>;
}

export interface EitherDiffRight<R> {
  t: EitherDiffIs.Right;
  v: DiffResult<R>;
}

export interface EitherDiffToLeft<L> {
  t: EitherDiffIs.ToLeft;
  v: DiffResult<L>;
}

export interface EitherDiffToRight<R> {
  t: EitherDiffIs.ToRight;
  v: DiffResult<R>;
}

export type EitherDiff<L, R> =
  | EitherDiffLeft<L>
  | EitherDiffRight<R>
  | EitherDiffToLeft<L>
  | EitherDiffToRight<R>;

export interface MayBeRefDiffRef {
  t: MayBeRefDiffIs.Ref;
  v: { $ref: string };
}

export interface MayBeRefDiffValue<T> {
  t: MayBeRefDiffIs.Value;
  v: DiffResult<T>;
}

export type MayBeRefDiff<T> = MayBeRefDiffRef | MayBeRefDiffValue<T>;

export function derefValue<T>(
  diff: MayBeRefDiff<T>,
  deref: (ref: string) => [string, DiffResult<T>] | undefined
): DiffResult<T> | undefined {
  switch (diff.t) {
    case MayBeRefDiffIs.Value:
      return diff.v;
    case MayBeRefDiffIs.Ref:
      const derefferenced = deref(diff.v.$ref);
      if (derefferenced) {
        return derefferenced[1];
      }
  }
}

export function getKey<T>(
  diff: MayBeRefDiff<T>,
  deref: (ref: string) => [string, DiffResult<T>] | undefined
): string | undefined {
  switch (diff.t) {
    case MayBeRefDiffIs.Value:
      return;
    case MayBeRefDiffIs.Ref:
      const derefferenced = deref(diff.v.$ref);
      if (derefferenced) {
        return derefferenced[0];
      }
  }
}

export type VecDiff<T> = Array<DiffResult<T>>;

export interface MapDiff<T> {
  [key: string]: DiffResult<T>;
}

export function getConcreteTypes(
  type: DiffResult<EitherDiff<string, VecDiff<string>>>
): string[] | undefined {
  const eitherPropertyType = valueOf(type);
  if (eitherPropertyType) {
    switch (eitherPropertyType.t) {
      case EitherDiffIs.Left:
        return [valueOf(eitherPropertyType.v)].filter(
          (value) => value
        ) as string[];
      case EitherDiffIs.Right:
        const types = valueOf(eitherPropertyType.v);
        if (types) {
          return types.map(valueOf).filter((value) => value) as string[];
        }
    }
  }
}
