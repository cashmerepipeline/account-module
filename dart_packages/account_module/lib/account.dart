import 'package:cashmere_core/entity/entity_base.dart';
import 'package:account_module/protocols/account_status.pbenum.dart';

/// 账号
class Account with EntityBase {
  String? areaCode;
  String? phone;
  String? password;
  String? email;
  LoginStatus? status;
  String? verificationCode;
  String? jwtToken;
  String? currentRoleGroup;

  Account();
}
