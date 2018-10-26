# ! [ allow ( unused_variables , unused_parens ) ] # ! [ cfg_attr ( feature = "cargo-clippy" , allow ( clippy :: float_cmp ) ) ] # ! [ cfg_attr ( feature = "cargo-clippy" , allow ( clippy :: unreadable_literal ) ) ] # ! [ cfg_attr ( feature = "cargo-clippy" , allow ( clippy :: nonminimal_bool ) ) ] extern crate matches ; use phf ; use super :: operands :: PluralOperands ; use super :: { PluralCategory , PluralRuleType } ; pub type PluralRule = fn ( & PluralOperands ) -> PluralCategory ; pub static CLDR_VERSION : usize = 0 ; # [ cfg_attr ( tarpaulin , skip ) ] pub fn get_locales ( pr_type : PluralRuleType ) -> & 'static [ & 'static str ] { match pr_type { PluralRuleType :: CARDINAL => & [ "test" ] } } # [ cfg_attr ( tarpaulin , skip ) ] pub fn get_pr ( lang_code : & str , pr_type : PluralRuleType ) -> Result < PluralRule , ( ) > { match pr_type { PluralRuleType :: CARDINAL => { static LANGUAGES : phf :: Map < & 'static str , PluralRule > = :: phf :: Map { key : 6246114685207409605 , disps : :: phf :: Slice :: Static ( & [ ( 0 , 0 ) , ] ) , entries : :: phf :: Slice :: Static ( & [ ( "test" , { fn rule_test ( po : & PluralOperands ) -> PluralCategory { if ( 2 . 0 <= po . n && po . n <= 10 . 0 && 8 > po . i && po . i > 9 ) { PluralCategory :: FEW } else if ( po . n == 1 . 0 ) { PluralCategory :: ONE } else if ( 1 <= po . i % 10 && po . i % 10 <= 2 ) { PluralCategory :: TWO } else { PluralCategory :: OTHER } } ; rule_test } ) , ] ) , } ; LANGUAGES . get ( lang_code ) . cloned ( ) . ok_or ( ( ) ) } } }