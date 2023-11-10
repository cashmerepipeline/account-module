import 'dart:async';

import 'package:account_module/account.dart';
import 'package:account_module/protocols/account_status.pbenum.dart';
import 'package:account_module/protocols/login.pb.dart';
import 'package:cashmere_core/grpc_call.dart';
import 'package:flutter/cupertino.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:account_module/login.dart' as m_login;
import 'package:jwt_decoder/jwt_decoder.dart';

class AccountNotifier extends StateNotifier<Account> {
  AccountNotifier(Account account) : super(account);

  void login(
    String areaCode,
    String phone,
    String password,
    GrpcCall<LoginRequest, LoginResponse> loginCall,
  ) async {
    state = Account();
    state.areaCode = areaCode;
    state.phone = phone;

    await m_login.login(areaCode, phone, password, loginCall).then((resp) {
      final newState = state;
      newState.jwtToken = resp.token;
      newState.status = LoginStatus.LoggedIn;
      final jwt = JwtDecoder.decode(resp.token);
      newState.currentRole = jwt["roles"][0].toString();
      state = newState;
    });
    debugPrint("login $state");
  }

  // 登录状态
  LoginStatus get loginStatus => state.status;

  // load cache
}

final accountProvider = StateNotifierProvider<AccountNotifier, Account>(
  (ref) => AccountNotifier(Account()),
);

class AccountAsyncNotifier extends AsyncNotifier<Account> {
  AccountAsyncNotifier(Account account) : super();

  void login(
    String areaCode,
    String phone,
    String password,
    GrpcCall<LoginRequest, LoginResponse> loginCall,
  ) async {
    final account = Account();
    account.areaCode = areaCode;
    account.phone = phone;

    await m_login.login(areaCode, phone, password, loginCall).then((resp) {
      account.jwtToken = resp.token;
      account.status = LoginStatus.LoggedIn;
      final jwt = JwtDecoder.decode(resp.token);
      account.currentRole = jwt["roles"][0].toString();

      state = AsyncValue.data(account);
    });
    debugPrint("login $account");
  }

  @override
  FutureOr<Account> build() {
    final account = Future.value(Account());
    return account;
  }
}

final accountAsyncNotifierProvider = AsyncNotifierProvider<AccountAsyncNotifier, Account>(
  () {
    return AccountAsyncNotifier(Account());
  },
);
