///
//  Generated code. Do not modify.
//  source: password.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,deprecated_member_use_from_same_package,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:core' as $core;
import 'dart:convert' as $convert;
import 'dart:typed_data' as $typed_data;
@$core.Deprecated('Use changeOwnPasswordRequestDescriptor instead')
const ChangeOwnPasswordRequest$json = const {
  '1': 'ChangeOwnPasswordRequest',
  '2': const [
    const {'1': 'old_password', '3': 1, '4': 1, '5': 9, '10': 'oldPassword'},
    const {'1': 'new_password', '3': 2, '4': 1, '5': 9, '10': 'newPassword'},
  ],
};

/// Descriptor for `ChangeOwnPasswordRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeOwnPasswordRequestDescriptor = $convert.base64Decode('ChhDaGFuZ2VPd25QYXNzd29yZFJlcXVlc3QSIQoMb2xkX3Bhc3N3b3JkGAEgASgJUgtvbGRQYXNzd29yZBIhCgxuZXdfcGFzc3dvcmQYAiABKAlSC25ld1Bhc3N3b3Jk');
@$core.Deprecated('Use changeOwnPasswordResponseDescriptor instead')
const ChangeOwnPasswordResponse$json = const {
  '1': 'ChangeOwnPasswordResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeOwnPasswordResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeOwnPasswordResponseDescriptor = $convert.base64Decode('ChlDaGFuZ2VPd25QYXNzd29yZFJlc3BvbnNlEhYKBnJlc3VsdBgBIAEoCVIGcmVzdWx0');
@$core.Deprecated('Use changeAccountPasswordRequestDescriptor instead')
const ChangeAccountPasswordRequest$json = const {
  '1': 'ChangeAccountPasswordRequest',
  '2': const [
    const {'1': 'account_id', '3': 1, '4': 1, '5': 9, '10': 'accountId'},
    const {'1': 'new_password', '3': 2, '4': 1, '5': 9, '10': 'newPassword'},
  ],
};

/// Descriptor for `ChangeAccountPasswordRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeAccountPasswordRequestDescriptor = $convert.base64Decode('ChxDaGFuZ2VBY2NvdW50UGFzc3dvcmRSZXF1ZXN0Eh0KCmFjY291bnRfaWQYASABKAlSCWFjY291bnRJZBIhCgxuZXdfcGFzc3dvcmQYAiABKAlSC25ld1Bhc3N3b3Jk');
@$core.Deprecated('Use changeAccountPasswordResponseDescriptor instead')
const ChangeAccountPasswordResponse$json = const {
  '1': 'ChangeAccountPasswordResponse',
  '2': const [
    const {'1': 'result', '3': 1, '4': 1, '5': 9, '10': 'result'},
  ],
};

/// Descriptor for `ChangeAccountPasswordResponse`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List changeAccountPasswordResponseDescriptor = $convert.base64Decode('Ch1DaGFuZ2VBY2NvdW50UGFzc3dvcmRSZXNwb25zZRIWCgZyZXN1bHQYASABKAlSBnJlc3VsdA==');
