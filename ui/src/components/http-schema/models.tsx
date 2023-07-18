import {
  VecDiff,
  MapDiff,
  DiffResult,
  MayBeRefDiff,
  EitherDiff,
} from "./common";

export interface HttpSchemaDiff {
  version: string;
  info?: DiffResult<InfoDiff>;
  servers?: DiffResult<VecDiff<ServerDiff>>;
  paths?: DiffResult<MapDiff<MayBeRefDiff<PathDiff>>>;
  components?: DiffResult<ComponentsDiff>;
  tags?: DiffResult<VecDiff<TagDiff>>;
  external_docs?: DiffResult<ExternalDocDiff>;
}

export interface InfoDiff {
  title?: DiffResult<string>;
  description?: DiffResult<string>;
  termsOfService?: DiffResult<string>;

  contact?: DiffResult<ContactDiff>;
  license?: DiffResult<LicenseDiff>;

  version?: DiffResult<string>;
}

export interface ContactDiff {
  name?: DiffResult<string>;
  url?: DiffResult<string>;
  email?: DiffResult<string>;
}

export interface LicenseDiff {
  name?: DiffResult<string>;
  url?: DiffResult<string>;
}

export interface ServerDiff {
  url?: DiffResult<string>;
  description?: DiffResult<string>;
  variables?: DiffResult<MapDiff<ServerVariableDiff>>;
}

export interface ServerVariableDiff {
  enum?: DiffResult<VecDiff<string>>;
  default?: DiffResult<any>;
  description?: DiffResult<string>;
}

export interface ComponentsDiff {
  schemas?: DiffResult<MapDiff<MayBeRefDiff<SchemaDiff>>>;
  responses?: DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>;
  parameters?: DiffResult<MapDiff<MayBeRefDiff<ParameterDiff>>>;
  examples?: DiffResult<MapDiff<MayBeRefDiff<ExampleDiff>>>;
  requestBodies?: DiffResult<MapDiff<MayBeRefDiff<RequestBodyDiff>>>;
  headers?: DiffResult<MapDiff<MayBeRefDiff<HeaderDiff>>>;
  securitySchemes?: DiffResult<MapDiff<MayBeRefDiff<SecuritySchemeDiff>>>;
  links?: DiffResult<MapDiff<MayBeRefDiff<LinkDiff>>>;
}

export interface ExternalDocDiff {
  url?: DiffResult<string>;
  description?: DiffResult<string>;
}

export interface ParameterDiff {
  name: string;
  in: string;
  description?: DiffResult<string>;
  required?: DiffResult<boolean>;
  deprecated: DiffResult<boolean>;
  allowEmptyValue?: DiffResult<boolean>;
  style?: DiffResult<string>;
  explode?: DiffResult<boolean>;
  allowReserved?: DiffResult<boolean>;

  schema?: DiffResult<MayBeRefDiff<SchemaDiff>>;

  examples?: DiffResult<MapDiff<MayBeRefDiff<any>>>;
  content?: DiffResult<MapDiff<MediaTypeDiff>>;

  customFields?: DiffResult<MapDiff<any>>;
}

export interface RequestBodyDiff {
  description?: DiffResult<string>;
  content?: DiffResult<MapDiff<MediaTypeDiff>>;
  required?: DiffResult<boolean>;
}

export interface MediaTypeDiff {
  schema?: DiffResult<MayBeRefDiff<SchemaDiff>>;
  examples?: DiffResult<MapDiff<MayBeRefDiff<ExampleDiff>>>;
  encoding?: DiffResult<MapDiff<EncodingDiff>>;
}

export interface EncodingDiff {
  contentType?: DiffResult<string>;
  headers?: DiffResult<MapDiff<MayBeRefDiff<HeaderDiff>>>;
  style?: DiffResult<string>;
  explode?: DiffResult<boolean>;
  allowReserved?: DiffResult<boolean>;
}

export interface LinkDiff {
  operationRef?: DiffResult<string>;
  operationId?: DiffResult<string>;
  parameters?: DiffResult<MapDiff<any>>;
  requestBody?: DiffResult<any>;
  description?: DiffResult<string>;
  server?: DiffResult<ServerDiff>;
}

export interface ResponseDiff {
  description?: DiffResult<string>;
  headers?: DiffResult<MapDiff<MayBeRefDiff<HeaderDiff>>>;
  content?: DiffResult<MapDiff<MediaTypeDiff>>;
  links?: DiffResult<MapDiff<MayBeRefDiff<LinkDiff>>>;
}

export interface ExampleDiff {
  summary?: DiffResult<string>;
  description?: DiffResult<string>;
  value?: DiffResult<any>;
  externalValue?: DiffResult<string>;
}

export interface DiscriminatorDiff {
  propertyName?: DiffResult<string>;
  mapping?: DiffResult<MapDiff<string>>;
}

export interface XMLDiff {
  name?: DiffResult<string>;
  namespace?: DiffResult<string>;
  prefix?: DiffResult<string>;
  attribute?: DiffResult<boolean>;
  wrapped?: DiffResult<boolean>;
}

export interface SecuritySchemeDiff {
  type?: DiffResult<string>;
  description?: DiffResult<string>;
  name?: DiffResult<string>;
  in?: DiffResult<string>;
  scheme?: DiffResult<string>;
  bearerFormat?: DiffResult<string>;
  flows?: DiffResult<OAuthFlowsDiff>;
  openIdConnectUrl: DiffResult<string>;
}

export interface OAuthFlowsDiff {
  implicit?: DiffResult<OAuthFlowDiff>;
  password?: DiffResult<OAuthFlowDiff>;
  clientCredentials?: DiffResult<OAuthFlowDiff>;
  authorizationCode?: DiffResult<OAuthFlowDiff>;
}

export interface OAuthFlowDiff {
  authorizationUrl?: DiffResult<string>;
  tokenUrl?: DiffResult<string>;
  refreshUrl?: DiffResult<string>;
  scopes?: DiffResult<MapDiff<string>>;
}

export interface TagDiff {
  name?: DiffResult<string>;
  description?: DiffResult<string>;
  externalDoc?: DiffResult<ExternalDocDiff>;
}

export interface SchemaDiff extends Record<string, any> {
  title?: DiffResult<string>;
  multipleOf?: DiffResult<number>;
  maximum?: DiffResult<number>;
  exclusiveMaximum?: DiffResult<boolean>;
  minimum?: DiffResult<number>;
  exclusiveMinimum?: DiffResult<boolean>;
  maxLength?: DiffResult<number>;
  minLength?: DiffResult<number>;
  pattern?: DiffResult<string>;
  maxItems?: DiffResult<number>;
  minItems?: DiffResult<number>;
  uniqueItems?: DiffResult<boolean>;
  maxProperties?: DiffResult<number>;
  minProperties?: DiffResult<number>;
  required?: DiffResult<VecDiff<string>>;
  enum?: DiffResult<VecDiff<any>>;

  type?: DiffResult<EitherDiff<string, VecDiff<string>>>;
  allOf?: DiffResult<VecDiff<MayBeRefDiff<SchemaDiff>>>;
  oneOf?: DiffResult<VecDiff<MayBeRefDiff<SchemaDiff>>>;
  anyOf?: DiffResult<VecDiff<MayBeRefDiff<SchemaDiff>>>;
  not?: DiffResult<VecDiff<MayBeRefDiff<SchemaDiff>>>;
  items?: DiffResult<MayBeRefDiff<SchemaDiff>>;
  properties?: DiffResult<MapDiff<MayBeRefDiff<SchemaDiff>>>;
  additionalProperties?: DiffResult<
    EitherDiff<boolean, MayBeRefDiff<SchemaDiff>>
  >;
  description?: DiffResult<string>;
  format?: DiffResult<string>;
  default?: DiffResult<any>;

  nullable?: DiffResult<boolean>;
  discriminator?: DiffResult<string>;
  readOnly?: DiffResult<boolean>;
  writeOnly?: DiffResult<boolean>;
  xml?: DiffResult<XMLDiff>;
  externalDocs?: DiffResult<ExternalDocDiff>;
  example?: DiffResult<string>;
  deprecated?: DiffResult<boolean>;

  customFields?: DiffResult<MapDiff<any>>;
}

export interface HeaderDiff {
  description?: DiffResult<string>;
  required?: DiffResult<boolean>;
  deprecated?: DiffResult<boolean>;
  allowEmptyValue?: DiffResult<boolean>;
  style?: DiffResult<string>;
  explode?: DiffResult<boolean>;
  allowReserved?: DiffResult<boolean>;

  schema?: DiffResult<MayBeRefDiff<SchemaDiff>>;

  examples?: DiffResult<MapDiff<MayBeRefDiff<any>>>;
  content?: DiffResult<MapDiff<MediaTypeDiff>>;

  customFields?: DiffResult<MapDiff<any>>;
}

export interface OperationDiff {
  tags?: DiffResult<VecDiff<string>>;
  summary?: DiffResult<string>;
  description?: DiffResult<string>;

  externalDocs?: DiffResult<ExternalDocDiff>;

  operationId?: DiffResult<string>;

  parameters?: DiffResult<VecDiff<MayBeRefDiff<ParameterDiff>>>;
  responses?: DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>;

  requestBody?: DiffResult<MayBeRefDiff<RequestBodyDiff>>;
  servers?: DiffResult<VecDiff<ServerDiff>>;

  deprecated?: DiffResult<boolean>;
}

export interface PathDiff {
  get?: DiffResult<OperationDiff>;
  put?: DiffResult<OperationDiff>;
  post?: DiffResult<OperationDiff>;
  delete?: DiffResult<OperationDiff>;
  options?: DiffResult<OperationDiff>;
  head?: DiffResult<OperationDiff>;
  patch?: DiffResult<OperationDiff>;
  trace?: DiffResult<OperationDiff>;

  servers?: DiffResult<VecDiff<ServerDiff>>;

  summary?: DiffResult<string>;
  description?: DiffResult<string>;
}
