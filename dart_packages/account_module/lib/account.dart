import 'package:cashmere_core/entity/entity_base.dart';
import 'package:account_module/protocols/account_status.pbenum.dart';
import 'package:jwt_decoder/jwt_decoder.dart';

/// 账号
class Account {
  String areaCode = "";
  String phone = "anonymous";
  String? jwtToken;

  LoginStatus status = LoginStatus.LoggedOut;

  String currentRole = "anonymous";
  List<String>? get roles {
    if (jwtToken == null) {
      return ["anonymous"];
    }

    final jwt = JwtDecoder.decode(jwtToken!);
    return jwt["roles"];
  }

  String? currentOrgnization;
  String? currentDepartment;

  String? get accountId {
    final result = "$areaCode$phone";
    return result.length > 0 ? result : null;
  }

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

  // to string
  @override
  String toString() {
    return "Account(areaCode: $areaCode, phone: $phone, jwtToken: $jwtToken, status: $status, currentRole: $currentRole, currentOrgnization: $currentOrgnization, currentDepartment: $currentDepartment)";
  }
}
