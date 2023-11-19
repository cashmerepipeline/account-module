import 'package:account_module/field_ids.dart';
import 'package:cashmere_core/entity/entity_base.dart';
import 'package:cashmere_core/view_models/entity_view_base.dart';
import 'package:cashmere_core/view_models/set_general_fields.dart';

class AccountView extends EntityViewBase {
  final String phone;
  final String password;

  AccountView(this.phone, this.password) : super(phone);

  factory AccountView.fromMap(Map<String, dynamic> map) {
    final account = AccountView(
      map[ACCOUNTS_PHONE_FIELD_ID.toString()],
      map[ACCOUNTS_PASSWORD_FIELD_ID.toString()],
    );
    final result = setGeneralFields(account, map);
    return result;
  }
}
