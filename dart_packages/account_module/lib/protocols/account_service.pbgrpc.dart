///
//  Generated code. Do not modify.
//  source: account_service.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

import 'dart:async' as $async;

import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'login.pb.dart' as $0;
import 'account.pb.dart' as $1;
import 'password.pb.dart' as $2;
export 'account_service.pb.dart';

class AccountGrpcClient extends $grpc.Client {
  static final _$login = $grpc.ClientMethod<$0.LoginRequest, $0.LoginResponse>(
      '/account_service.AccountGrpc/Login',
      ($0.LoginRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) => $0.LoginResponse.fromBuffer(value));
  static final _$newAccount =
      $grpc.ClientMethod<$1.NewAccountRequest, $1.NewAccountResponse>(
          '/account_service.AccountGrpc/NewAccount',
          ($1.NewAccountRequest value) => value.writeToBuffer(),
          ($core.List<$core.int> value) =>
              $1.NewAccountResponse.fromBuffer(value));
  static final _$addAccountIntoGroup = $grpc.ClientMethod<
          $1.AddAccountIntoGroupRequest, $1.AddAccountIntoGroupResponse>(
      '/account_service.AccountGrpc/AddAccountIntoGroup',
      ($1.AddAccountIntoGroupRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $1.AddAccountIntoGroupResponse.fromBuffer(value));
  static final _$removeAccountFromGroup = $grpc.ClientMethod<
          $1.RemoveAccountFromGroupRequest, $1.RemoveAccountFromGroupResponse>(
      '/account_service.AccountGrpc/RemoveAccountFromGroup',
      ($1.RemoveAccountFromGroupRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $1.RemoveAccountFromGroupResponse.fromBuffer(value));
  static final _$changeOwnPassword = $grpc.ClientMethod<
          $2.ChangeOwnPasswordRequest, $2.ChangeOwnPasswordResponse>(
      '/account_service.AccountGrpc/ChangeOwnPassword',
      ($2.ChangeOwnPasswordRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $2.ChangeOwnPasswordResponse.fromBuffer(value));
  static final _$changeAccountStatus = $grpc.ClientMethod<
          $1.ChangeAccountStatusRequest, $1.ChangeAccountStatusResponse>(
      '/account_service.AccountGrpc/ChangeAccountStatus',
      ($1.ChangeAccountStatusRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $1.ChangeAccountStatusResponse.fromBuffer(value));
  static final _$changeAccountPassword = $grpc.ClientMethod<
          $2.ChangeAccountPasswordRequest, $2.ChangeAccountPasswordResponse>(
      '/account_service.AccountGrpc/ChangeAccountPassword',
      ($2.ChangeAccountPasswordRequest value) => value.writeToBuffer(),
      ($core.List<$core.int> value) =>
          $2.ChangeAccountPasswordResponse.fromBuffer(value));

  AccountGrpcClient($grpc.ClientChannel channel,
      {$grpc.CallOptions? options,
      $core.Iterable<$grpc.ClientInterceptor>? interceptors})
      : super(channel, options: options, interceptors: interceptors);

  $grpc.ResponseFuture<$0.LoginResponse> login($0.LoginRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$login, request, options: options);
  }

  $grpc.ResponseFuture<$1.NewAccountResponse> newAccount(
      $1.NewAccountRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$newAccount, request, options: options);
  }

  $grpc.ResponseFuture<$1.AddAccountIntoGroupResponse> addAccountIntoGroup(
      $1.AddAccountIntoGroupRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$addAccountIntoGroup, request, options: options);
  }

  $grpc.ResponseFuture<$1.RemoveAccountFromGroupResponse>
      removeAccountFromGroup($1.RemoveAccountFromGroupRequest request,
          {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$removeAccountFromGroup, request,
        options: options);
  }

  $grpc.ResponseFuture<$2.ChangeOwnPasswordResponse> changeOwnPassword(
      $2.ChangeOwnPasswordRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$changeOwnPassword, request, options: options);
  }

  $grpc.ResponseFuture<$1.ChangeAccountStatusResponse> changeAccountStatus(
      $1.ChangeAccountStatusRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$changeAccountStatus, request, options: options);
  }

  $grpc.ResponseFuture<$2.ChangeAccountPasswordResponse> changeAccountPassword(
      $2.ChangeAccountPasswordRequest request,
      {$grpc.CallOptions? options}) {
    return $createUnaryCall(_$changeAccountPassword, request, options: options);
  }
}

abstract class AccountGrpcServiceBase extends $grpc.Service {
  $core.String get $name => 'account_service.AccountGrpc';

  AccountGrpcServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.LoginRequest, $0.LoginResponse>(
        'Login',
        login_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.LoginRequest.fromBuffer(value),
        ($0.LoginResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.NewAccountRequest, $1.NewAccountResponse>(
        'NewAccount',
        newAccount_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $1.NewAccountRequest.fromBuffer(value),
        ($1.NewAccountResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.AddAccountIntoGroupRequest,
            $1.AddAccountIntoGroupResponse>(
        'AddAccountIntoGroup',
        addAccountIntoGroup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $1.AddAccountIntoGroupRequest.fromBuffer(value),
        ($1.AddAccountIntoGroupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.RemoveAccountFromGroupRequest,
            $1.RemoveAccountFromGroupResponse>(
        'RemoveAccountFromGroup',
        removeAccountFromGroup_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $1.RemoveAccountFromGroupRequest.fromBuffer(value),
        ($1.RemoveAccountFromGroupResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$2.ChangeOwnPasswordRequest,
            $2.ChangeOwnPasswordResponse>(
        'ChangeOwnPassword',
        changeOwnPassword_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $2.ChangeOwnPasswordRequest.fromBuffer(value),
        ($2.ChangeOwnPasswordResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$1.ChangeAccountStatusRequest,
            $1.ChangeAccountStatusResponse>(
        'ChangeAccountStatus',
        changeAccountStatus_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $1.ChangeAccountStatusRequest.fromBuffer(value),
        ($1.ChangeAccountStatusResponse value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$2.ChangeAccountPasswordRequest,
            $2.ChangeAccountPasswordResponse>(
        'ChangeAccountPassword',
        changeAccountPassword_Pre,
        false,
        false,
        ($core.List<$core.int> value) =>
            $2.ChangeAccountPasswordRequest.fromBuffer(value),
        ($2.ChangeAccountPasswordResponse value) => value.writeToBuffer()));
  }

  $async.Future<$0.LoginResponse> login_Pre(
      $grpc.ServiceCall call, $async.Future<$0.LoginRequest> request) async {
    return login(call, await request);
  }

  $async.Future<$1.NewAccountResponse> newAccount_Pre($grpc.ServiceCall call,
      $async.Future<$1.NewAccountRequest> request) async {
    return newAccount(call, await request);
  }

  $async.Future<$1.AddAccountIntoGroupResponse> addAccountIntoGroup_Pre(
      $grpc.ServiceCall call,
      $async.Future<$1.AddAccountIntoGroupRequest> request) async {
    return addAccountIntoGroup(call, await request);
  }

  $async.Future<$1.RemoveAccountFromGroupResponse> removeAccountFromGroup_Pre(
      $grpc.ServiceCall call,
      $async.Future<$1.RemoveAccountFromGroupRequest> request) async {
    return removeAccountFromGroup(call, await request);
  }

  $async.Future<$2.ChangeOwnPasswordResponse> changeOwnPassword_Pre(
      $grpc.ServiceCall call,
      $async.Future<$2.ChangeOwnPasswordRequest> request) async {
    return changeOwnPassword(call, await request);
  }

  $async.Future<$1.ChangeAccountStatusResponse> changeAccountStatus_Pre(
      $grpc.ServiceCall call,
      $async.Future<$1.ChangeAccountStatusRequest> request) async {
    return changeAccountStatus(call, await request);
  }

  $async.Future<$2.ChangeAccountPasswordResponse> changeAccountPassword_Pre(
      $grpc.ServiceCall call,
      $async.Future<$2.ChangeAccountPasswordRequest> request) async {
    return changeAccountPassword(call, await request);
  }

  $async.Future<$0.LoginResponse> login(
      $grpc.ServiceCall call, $0.LoginRequest request);
  $async.Future<$1.NewAccountResponse> newAccount(
      $grpc.ServiceCall call, $1.NewAccountRequest request);
  $async.Future<$1.AddAccountIntoGroupResponse> addAccountIntoGroup(
      $grpc.ServiceCall call, $1.AddAccountIntoGroupRequest request);
  $async.Future<$1.RemoveAccountFromGroupResponse> removeAccountFromGroup(
      $grpc.ServiceCall call, $1.RemoveAccountFromGroupRequest request);
  $async.Future<$2.ChangeOwnPasswordResponse> changeOwnPassword(
      $grpc.ServiceCall call, $2.ChangeOwnPasswordRequest request);
  $async.Future<$1.ChangeAccountStatusResponse> changeAccountStatus(
      $grpc.ServiceCall call, $1.ChangeAccountStatusRequest request);
  $async.Future<$2.ChangeAccountPasswordResponse> changeAccountPassword(
      $grpc.ServiceCall call, $2.ChangeAccountPasswordRequest request);
}
