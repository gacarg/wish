# wish


write your wishes on near contract!
_____
  for build and dev-deploy(use helper):
 'run build.sh'
 # commands:
 add wish:
 'near call U_CONTRACT  --accountId=U_ACCOUNT_UD add_wish '{"wish":"(U_WISH)"}' --deposit TIPS
 get wish:
 ''
 get list wish: 
 'near view U_CONTRACT  --accountId=U_ACCOUNT_UD get_list_wish'
 
 get wish
 'near call U_CONTRACT  --accountId=U_ACCOUNT_UD '{"account_id":"SEARCH_ACCOUNT_ID"}' '
 
