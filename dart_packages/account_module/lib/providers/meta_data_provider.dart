import 'package:account_module/providers/account_provider.dart';
import 'package:cashmere_core/auth_codes.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final metaDataFutureProvider = FutureProvider<Map<String, String>>((ref) async {
  final account = await ref.watch(accountAsyncNotifierProvider.future);
  final jwtToken = account.jwtToken;
  final roleGroup = account.currentRole;

  final metaData = <String, String>{};
  metaData[RoleGroupName] = roleGroup ?? "";
  metaData['authorization'] = 'Bearer $jwtToken';
  return metaData;
});
