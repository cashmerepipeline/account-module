///
//  Generated code. Do not modify.
//  source: account_status.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class LoginStatus extends $pb.ProtobufEnum {
  static const LoginStatus LoggedIn = LoginStatus._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LoggedIn');
  static const LoginStatus LoggedOut = LoginStatus._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LoggedOut');
  static const LoginStatus LogginFailed = LoginStatus._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'LogginFailed');

  static const $core.List<LoginStatus> values = <LoginStatus> [
    LoggedIn,
    LoggedOut,
    LogginFailed,
  ];

  static final $core.Map<$core.int, LoginStatus> _byValue = $pb.ProtobufEnum.initByValue(values);
  static LoginStatus? valueOf($core.int value) => _byValue[value];

  const LoginStatus._($core.int v, $core.String n) : super(v, n);
}

class AccountStatus extends $pb.ProtobufEnum {
  static const AccountStatus AccountStopped = AccountStatus._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'AccountStopped');
  static const AccountStatus AccountActived = AccountStatus._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'AccountActived');

  static const $core.List<AccountStatus> values = <AccountStatus> [
    AccountStopped,
    AccountActived,
  ];

  static final $core.Map<$core.int, AccountStatus> _byValue = $pb.ProtobufEnum.initByValue(values);
  static AccountStatus? valueOf($core.int value) => _byValue[value];

  const AccountStatus._($core.int v, $core.String n) : super(v, n);
}

