///
//  Generated code. Do not modify.
//  source: account.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use newAccountRequestDescriptor instead')
const NewAccountRequest$json = const {
  '1': 'NewAccountRequest',
  '2': const [
    const {'1': 'area_code', '3': 1, '4': 1, '5': 9, '10': 'areaCode'},
    const {'1': 'phone', '3': 2, '4': 1, '5': 9, '10': 'phone'},
    const {'1': 'password', '3': 3, '4': 1, '5': 9, '10': 'password'},
    const {'1': 'nick_name', '3': 4, '4': 1, '5': 11, '6': '.cashmere.Name', '10': 'nickName'},
  ],
};

/// Descriptor for `NewAccountRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newAccountRequestDescriptor = $convert.base64Decode('ChFOZXdBY2NvdW50UmVxdWVzdBIbCglhcmVhX2NvZGUYASABKAlSCGFyZWFDb2RlEhQKBXBob25lGAIgASgJUgVwaG9uZRIaCghwYXNzd29yZBgDIAEoCVIIcGFzc3dvcmQSKwoJbmlja19uYW1lGAQgASgLMg4uY2FzaG1lcmUuTmFtZVIIbmlja05hbWU=');
@$core.Deprecated('Use newAccountResponseDescriptor instead')
const NewAccountResponse$json = const {
  '1': 'NewAccountResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `NewAccountResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List newAccountResponseDescriptor = $convert.base64Decode('ChJOZXdBY2NvdW50UmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use registerRequestDescriptor instead')
const RegisterRequest$json = const {
  '1': 'RegisterRequest',
  '2': const [
    const {'1': 'organization_id', '3': 1, '4': 1, '5': 9, '10': 'organizationId'},
    const {'1': 'department_id', '3': 2, '4': 1, '5': 9, '10': 'departmentId'},
    const {'1': 'phone', '3': 4, '4': 1, '5': 9, '10': 'phone'},
    const {'1': 'password', '3': 5, '4': 1, '5': 9, '10': 'password'},
  ],
};

/// Descriptor for `RegisterRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerRequestDescriptor = $convert.base64Decode('Cg9SZWdpc3RlclJlcXVlc3QSJwoPb3JnYW5pemF0aW9uX2lkGAEgASgJUg5vcmdhbml6YXRpb25JZBIjCg1kZXBhcnRtZW50X2lkGAIgASgJUgxkZXBhcnRtZW50SWQSFAoFcGhvbmUYBCABKAlSBXBob25lEhoKCHBhc3N3b3JkGAUgASgJUghwYXNzd29yZA==');
@$core.Deprecated('Use registerResponseDescriptor instead')
const RegisterResponse$json = const {
  '1': 'RegisterResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RegisterResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List registerResponseDescriptor = $convert.base64Decode('ChBSZWdpc3RlclJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use changeOwnPhoneRequestDescriptor instead')
const ChangeOwnPhoneRequest$json = const {
  '1': 'ChangeOwnPhoneRequest',
  '2': const [
    const {'1': 'old_phone', '3': 1, '4': 1, '5': 9, '10': 'oldPhone'},
    const {'1': 'new_phone', '3': 2, '4': 1, '5': 9, '10': 'newPhone'},
    const {'1': 'password', '3': 3, '4': 1, '5': 9, '10': 'password'},
  ],
};

/// Descriptor for `ChangeOwnPhoneRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeOwnPhoneRequestDescriptor = $convert.base64Decode('ChVDaGFuZ2VPd25QaG9uZVJlcXVlc3QSGwoJb2xkX3Bob25lGAEgASgJUghvbGRQaG9uZRIbCgluZXdfcGhvbmUYAiABKAlSCG5ld1Bob25lEhoKCHBhc3N3b3JkGAMgASgJUghwYXNzd29yZA==');
@$core.Deprecated('Use changePhoneOwnResponseDescriptor instead')
const ChangePhoneOwnResponse$json = const {
  '1': 'ChangePhoneOwnResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangePhoneOwnResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changePhoneOwnResponseDescriptor = $convert.base64Decode('ChZDaGFuZ2VQaG9uZU93blJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use addAccountIntoGroupRequestDescriptor instead')
const AddAccountIntoGroupRequest$json = const {
  '1': 'AddAccountIntoGroupRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 9, '10': 'accountId'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
  ],
};

/// Descriptor for `AddAccountIntoGroupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addAccountIntoGroupRequestDescriptor = $convert.base64Decode('ChpBZGRBY2NvdW50SW50b0dyb3VwUmVxdWVzdBIdCgphY2NvdW50X2lkGAEgASgJUglhY2NvdW50SWQSGQoIZ3JvdXBfaWQYAiABKAlSB2dyb3VwSWQ=');
@$core.Deprecated('Use addAccountIntoGroupResponseDescriptor instead')
const AddAccountIntoGroupResponse$json = const {
  '1': 'AddAccountIntoGroupResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `AddAccountIntoGroupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List addAccountIntoGroupResponseDescriptor = $convert.base64Decode('ChtBZGRBY2NvdW50SW50b0dyb3VwUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use removeAccountFromGroupRequestDescriptor instead')
const RemoveAccountFromGroupRequest$json = const {
  '1': 'RemoveAccountFromGroupRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 9, '10': 'accountId'},
    const {'1': 'group_id', '3': 2, '4': 1, '5': 9, '10': 'groupId'},
  ],
};

/// Descriptor for `RemoveAccountFromGroupRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeAccountFromGroupRequestDescriptor = $convert.base64Decode('Ch1SZW1vdmVBY2NvdW50RnJvbUdyb3VwUmVxdWVzdBIdCgphY2NvdW50X2lkGAEgASgJUglhY2NvdW50SWQSGQoIZ3JvdXBfaWQYAiABKAlSB2dyb3VwSWQ=');
@$core.Deprecated('Use removeAccountFromGroupResponseDescriptor instead')
const RemoveAccountFromGroupResponse$json = const {
  '1': 'RemoveAccountFromGroupResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `RemoveAccountFromGroupResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List removeAccountFromGroupResponseDescriptor = $convert.base64Decode('Ch5SZW1vdmVBY2NvdW50RnJvbUdyb3VwUmVzcG9uc2USFgoGcmVzdWx0GAEgASgJUgZyZXN1bHQ=');
@$core.Deprecated('Use changeAccountStatusRequestDescriptor instead')
const ChangeAccountStatusRequest$json = const {
  '1': 'ChangeAccountStatusRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 9, '10': 'accountId'},
    const {'1': 'status', '3': 2, '4': 1, '5': 14, '6': '.account_service.AccountStatus', '10': 'status'},
  ],
};

/// Descriptor for `ChangeAccountStatusRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeAccountStatusRequestDescriptor = $convert.base64Decode('ChpDaGFuZ2VBY2NvdW50U3RhdHVzUmVxdWVzdBIdCgphY2NvdW50X2lkGAEgASgJUglhY2NvdW50SWQSNgoGc3RhdHVzGAIgASgOMh4uYWNjb3VudF9zZXJ2aWNlLkFjY291bnRTdGF0dXNSBnN0YXR1cw==');
@$core.Deprecated('Use changeAccountStatusResponseDescriptor instead')
const ChangeAccountStatusResponse$json = const {
  '1': 'ChangeAccountStatusResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 14, '6': '.account_service.AccountStatus', '10': 'result'},
  ],
};

/// Descriptor for `ChangeAccountStatusResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeAccountStatusResponseDescriptor = $convert.base64Decode('ChtDaGFuZ2VBY2NvdW50U3RhdHVzUmVzcG9uc2USNgoGcmVzdWx0GAEgASgOMh4uYWNjb3VudF9zZXJ2aWNlLkFjY291bnRTdGF0dXNSBnJlc3VsdA==');
