import 'package:cashmere_core/entity/entity_base.dart';
import 'package:account_module/protocols/account_status.pbenum.dart';
import 'package:jwt_decoder/jwt_decoder.dart';

/// 账号
class Account {
  String? areaCode;
  String? phone;
  String? jwtToken;

  LoginStatus status = LoginStatus.LoggedOut;

  String? currentRole;
  List<String>? get roles {
    return jwtToken ?? JwtDecoder.decode(jwtToken!)["roles"];
  }

  String? currentOrgnization;
  String? currentDepartment;

  String? get systemAdminRole {
    return jwtToken ?? JwtDecoder.decode(jwtToken!)["systemAdminRole"];
  }

  String? get orgnizationAdminRole {
    return jwtToken ?? JwtDecoder.decode(jwtToken!)["organizationAdminRole"];
  }

  String? get departmentAdminRole {
    return jwtToken ?? JwtDecoder.decode(jwtToken!)["departmentAdminRole"];
  }

  String? get groupAdminRole {
    return jwtToken ?? JwtDecoder.decode(jwtToken!)["groupAdminRole"];
  }

  Account();
}
