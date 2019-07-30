// Copyright 2018 Chainpool.

use super::*;
use mock::*;
use runtime_io::with_externalities;

#[test]
fn test_genesis() {
    with_externalities(&mut new_test_ext(), || {
        // Check that GenesisBuilder works properly.
        // check token_list
        let btc_symbol = b"x-btc".to_vec();
        let eth_symbol = b"x-eth".to_vec();

        assert_eq!(
            TokenBalances::token_list(),
            vec![
                Test::CHAINX_SYMBOL.to_vec(),
                btc_symbol.clone(),
                eth_symbol.clone(),
            ]
        );

        assert_eq!(
            TokenBalances::token_info(btc_symbol.clone())
                .unwrap()
                .0
                .precision(),
            8
        );
        assert_eq!(
            TokenBalances::token_info(eth_symbol.clone())
                .unwrap()
                .0
                .precision(),
            4
        );

        assert_eq!(TokenBalances::total_free_token(btc_symbol.clone()), 100);
        assert_eq!(TokenBalances::total_reserved_token(btc_symbol.clone()), 0);

        // chainx symbol for every user
        assert_eq!(
            TokenBalances::token_list_of(&0),
            [Test::CHAINX_SYMBOL.to_vec()].to_vec()
        );
    });
}

#[test]
fn test_genesis_token_issue() {
    with_externalities(&mut new_test_ext(), || {
        let btc_symbol = b"x-btc".to_vec();
        let eth_symbol = b"x-eth".to_vec();
        assert_eq!(
            TokenBalances::free_token(&(3, Test::CHAINX_SYMBOL.to_vec())),
            1000
        );
        assert_eq!(TokenBalances::free_token(&(3, btc_symbol.clone())), 100);
        assert_eq!(TokenBalances::free_token(&(3, eth_symbol.clone())), 100);

        assert_eq!(
            TokenBalances::token_list_of(&3),
            [Test::CHAINX_SYMBOL.to_vec(), btc_symbol, eth_symbol]
        );
    })
}

#[test]
#[should_panic]
fn test_err_genesis() {
    with_externalities(&mut err_test_ext(), || {})
}

#[test]
fn test_register() {
    with_externalities(&mut new_test_ext(), || {
        let t_sym: Symbol = b"x-eos".to_vec(); //slice_to_u8_8(b"x-eos");
        let t_desc: TokenDesc = b"eos token".to_vec(); //slice_to_u8_32(b"eos token");
        let precision = 4;
        let t: Token = Token::new(t_sym.clone(), t_desc, precision);
        assert_eq!(TokenBalances::register_token(t, 0, 0), Ok(()));

        assert_eq!(TokenBalances::token_list_len(), 4);
        assert_eq!(TokenBalances::token_list_map(3), t_sym.clone());

        let btc_symbol = b"x-btc".to_vec(); //b"x-btc".to_vec();
        let eth_symbol = b"x-eth".to_vec(); //slice_to_u8_8(b"x-eth");
        assert_eq!(
            TokenBalances::token_list(),
            vec![
                Test::CHAINX_SYMBOL.to_vec(),
                btc_symbol.clone(),
                eth_symbol.clone(),
                t_sym.clone(),
            ]
        );

        assert_eq!(TokenBalances::total_free_token(t_sym.clone()), 0);
        assert_eq!(
            TokenBalances::token_info(t_sym.clone())
                .unwrap()
                .0
                .precision(),
            4
        );

        // test err branch
        let btc_t = Token::new(btc_symbol.clone(), b"btc token".to_vec(), 4);
        assert_noop!(
            TokenBalances::register_token(btc_t, 0, 0),
            "already has this token symbol"
        );
        assert_eq!(TokenBalances::token_list_len(), 4);
        assert_eq!(TokenBalances::token_list_map(4), b"".to_vec());
    })
}

#[test]
fn test_remove() {
    with_externalities(&mut new_test_ext(), || {
        // register a new token
        let t_sym: Symbol = b"x-eos".to_vec();
        let t_desc: TokenDesc = b"eos token".to_vec();
        let precision: Precision = 4;
        let t: Token = Token::new(t_sym.clone(), t_desc, precision);
        assert_eq!(TokenBalances::register_token(t.clone(), 0, 0), Ok(()));
        assert_eq!(TokenBalances::token_list_map(3), t_sym.clone());

        // remove it
        assert_eq!(TokenBalances::cancel_token(&t_sym.clone()), Ok(()));
        assert_eq!(TokenBalances::token_list_map(3), t_sym.clone());
        assert_eq!(TokenBalances::token_list_len(), 4); // length not modify

        // re-register, but must be failed
        assert_noop!(
            TokenBalances::register_token(t.clone(), 0, 0),
            "already has this token symbol"
        );

        // create new token symbol
        let t_new: Token = Token {
            symbol: b"x-eos2".to_vec(),
            ..t
        };
        assert_noop!(
            TokenBalances::cancel_token(&t_new.symbol),
            "this token symbol dose not register yet or is invalid"
        );
        assert_eq!(TokenBalances::register_token(t_new.clone(), 0, 0), Ok(()));
        assert_eq!(TokenBalances::token_list_map(3), t_sym.clone());
        assert_eq!(TokenBalances::token_list_map(4), t_new.symbol);
        assert_eq!(TokenBalances::token_list_len(), 5);
    })
}

#[test]
fn test_total_balance() {
    with_externalities(&mut new_test_ext(), || {
        let btc_symbol = b"x-btc".to_vec();
        assert_eq!(TokenBalances::total_token(&btc_symbol.clone()), 100);

        TokenBalances::issue(&0, &btc_symbol, 100).unwrap();
        assert_eq!(TokenBalances::total_token(&btc_symbol.clone()), 200);

        TokenBalances::issue(&0, &btc_symbol, 50).unwrap();
        TokenBalances::reserve(&0, &btc_symbol, 50, Default::default()).unwrap();
        assert_eq!(TokenBalances::total_token(&btc_symbol.clone()), 250);

        TokenBalances::destroy(&0, &btc_symbol, 25, Default::default()).unwrap();
        assert_eq!(TokenBalances::total_token(&btc_symbol.clone()), 225);
    })
}

#[test]
fn test_account_balance() {
    with_externalities(&mut new_test_ext(), || {
        let a: u64 = 1; // accountid
        let btc_symbol = b"x-btc".to_vec();
        let key = (a, btc_symbol.clone());
        let reserved_key = (a, btc_symbol.clone(), Default::default());
        assert_eq!(TokenBalances::free_token(&key), 0);
        assert_eq!(TokenBalances::reserved_token(&reserved_key), 0);
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol), 0);

        TokenBalances::issue(&a, &btc_symbol, 100).unwrap();
        assert_eq!(TokenBalances::free_token(&key), 100);
        assert_eq!(TokenBalances::reserved_token(&reserved_key), 0);
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 100);

        TokenBalances::reserve(&a, &btc_symbol, 50, Default::default()).unwrap();
        TokenBalances::destroy(&a, &btc_symbol, 50, Default::default()).unwrap();
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 50);
    })
}

#[test]
fn test_normal_issue_and_destroy() {
    with_externalities(&mut new_test_ext(), || {
        let a: u64 = 1; // accountid
        let btc_symbol = b"x-btc".to_vec();
        let key = (a, btc_symbol.clone());
        let reserved_key = (a, btc_symbol.clone(), Default::default());

        // issue
        TokenBalances::issue(&a, &btc_symbol.clone(), 50).unwrap();
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 50);
        assert_eq!(TokenBalances::total_token(&btc_symbol.clone()), 150);

        // reserve
        TokenBalances::reserve(&a, &btc_symbol.clone(), 25, Default::default()).unwrap();
        assert_eq!(TokenBalances::reserved_token(&reserved_key), 25);
        assert_eq!(TokenBalances::free_token(&key), 25);
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 50);
        assert_eq!(TokenBalances::total_reserved_token(&btc_symbol.clone()), 25);

        // destroy
        TokenBalances::destroy(&a, &btc_symbol.clone(), 25, Default::default()).unwrap();
        assert_eq!(TokenBalances::reserved_token(&reserved_key), 0);
        assert_eq!(TokenBalances::free_token(&key), 25);
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 25);
        assert_eq!(TokenBalances::total_reserved_token(&btc_symbol.clone()), 0);
        assert_eq!(TokenBalances::total_token(&btc_symbol.clone()), 125);
    })
}

#[test]
fn test_unlock_issue_and_destroy2() {
    with_externalities(&mut new_test_ext(), || {
        let a: u64 = 1; // accountid
        let btc_symbol = b"x-btc".to_vec();
        let key = (a, btc_symbol.clone());
        let reserved_key = (a, btc_symbol.clone(), Default::default());

        // issue
        TokenBalances::issue(&a, &btc_symbol.clone(), 50).unwrap();
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 50);
        assert_eq!(TokenBalances::total_token(&btc_symbol.clone()), 150);

        // reserve
        TokenBalances::reserve(&a, &btc_symbol.clone(), 25, Default::default()).unwrap();
        assert_eq!(TokenBalances::reserved_token(&reserved_key), 25);
        assert_eq!(TokenBalances::free_token(&key), 25);
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 50);
        assert_eq!(TokenBalances::total_reserved_token(&btc_symbol.clone()), 25);

        // unreserve
        TokenBalances::unreserve(&a, &btc_symbol.clone(), 10, Default::default()).unwrap();
        assert_eq!(TokenBalances::reserved_token(&reserved_key), 15);
        assert_eq!(TokenBalances::free_token(&key), 35);
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 50);
        assert_eq!(TokenBalances::total_reserved_token(&btc_symbol.clone()), 15);
    })
}

#[test]
fn test_error_issue_and_destroy1() {
    with_externalities(&mut new_test_ext(), || {
        let a: u64 = 1; // accountid
        let btc_symbol = b"x-btc".to_vec();
        // issue
        TokenBalances::issue(&a, &btc_symbol.clone(), 50).unwrap();
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 50);
        assert_eq!(TokenBalances::total_token(&btc_symbol.clone()), 150);
        // destroy first
        // destroy
        assert_err!(
            TokenBalances::destroy(&a, &btc_symbol.clone(), 25, Default::default()),
            "reserved token too low to destroy"
        );
        // reserve
        assert_eq!(TokenBalances::total_free_token(&btc_symbol.clone()), 150);
        assert_err!(
            TokenBalances::reserve(&a, &btc_symbol.clone(), 100, Default::default()),
            "free token too low to reserve"
        );
        // lock first
        assert_ok!(TokenBalances::reserve(
            &a,
            &btc_symbol.clone(),
            25,
            Default::default()
        ));
        // destroy
        assert_ok!(TokenBalances::destroy(
            &a,
            &btc_symbol.clone(),
            25,
            Default::default()
        ));
    })
}

#[test]
fn test_error_issue_and_destroy2() {
    with_externalities(&mut new_test_ext(), || {
        let a: u64 = 1; // accountid
        let btc_symbol = b"x-btc".to_vec();
        // issue
        TokenBalances::issue(&a, &btc_symbol.clone(), 50).unwrap();
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol), 50);
        assert_eq!(TokenBalances::total_token(&btc_symbol.clone()), 150);
        // overflow
        let i: i32 = -1;
        assert_err!(
            TokenBalances::reserve(
                &a,
                &btc_symbol.clone(),
                i as TokenBalance,
                Default::default()
            ),
            "free token too low to reserve"
        );
        assert_err!(
            TokenBalances::issue(&a, &btc_symbol.clone(), i as TokenBalance),
            "free token too high to issue"
        );
    })
}

#[test]
fn test_error_issue_and_destroy3() {
    with_externalities(&mut new_test_ext(), || {
        let a: u64 = 1; // accountid
        let btc_symbol = b"x-btc".to_vec();
        // lock or destroy without init
        assert_err!(
            TokenBalances::destroy(&a, &btc_symbol.clone(), 25, Default::default()),
            "not a existed token in this account token list"
        );
        assert_err!(
            TokenBalances::reserve(&a, &btc_symbol.clone(), 25, Default::default()),
            "not a existed token in this account token list"
        );
        TokenBalances::issue(&a, &btc_symbol.clone(), 0).unwrap();
        assert_err!(
            TokenBalances::destroy(&a, &btc_symbol.clone(), 25, Default::default()),
            "reserved token too low to destroy"
        );
        assert_err!(
            TokenBalances::reserve(&a, &btc_symbol.clone(), 25, Default::default()),
            "free token too low to reserve"
        );

        TokenBalances::issue(&a, &btc_symbol.clone(), 100).unwrap();
        assert_ok!(TokenBalances::reserve(
            &a,
            &btc_symbol.clone(),
            25,
            Default::default()
        ));
        assert_ok!(TokenBalances::destroy(
            &a,
            &btc_symbol.clone(),
            25,
            Default::default()
        ));
    })
}

#[test]
fn test_transfer_not_init() {
    with_externalities(&mut new_test_ext2(), || {
        let a: u64 = 1; // accountid
        let new_id: u64 = 100;
        let btc_symbol = b"x-btc".to_vec();
        TokenBalances::issue(&a, &btc_symbol.clone(), 50).unwrap();
        assert_ok!(TokenBalances::transfer(
            Some(a).into(),
            new_id.into(),
            btc_symbol.clone(),
            25
        ));
        assert_eq!(Balances::lookup_index(3), Some(new_id));
        assert_err!(
            associations::Module::<Test>::init_account(Some(a).into(), new_id.into(),),
            "this account is existing"
        );
        assert_ok!(TokenBalances::transfer(
            Some(a).into(),
            new_id.into(),
            btc_symbol.clone(),
            25
        ));
        assert_ok!(TokenBalances::transfer(
            Some(a).into(),
            new_id.into(),
            Test::CHAINX_SYMBOL.to_vec(),
            25
        ));

        assert_eq!(
            TokenBalances::free_token(&(a, Test::CHAINX_SYMBOL.to_vec())),
            1000 - 10 - 10 - 25 - 10
        );
        assert_eq!(
            TokenBalances::free_token(&(new_id, Test::CHAINX_SYMBOL.to_vec())),
            25
        );
    })
}

#[test]
fn test_transfer_chainx() {
    with_externalities(&mut new_test_ext2(), || {
        let a: u64 = 1; // accountid
        let b: u64 = 2; // accountid
        assert_ok!(TokenBalances::transfer(
            Some(a).into(),
            b.into(),
            Test::CHAINX_SYMBOL.to_vec(),
            25
        ));

        assert_eq!(
            TokenBalances::free_token(&(a, Test::CHAINX_SYMBOL.to_vec())),
            1000 - 10 - 25
        );
        assert_eq!(
            TokenBalances::free_token(&(b, Test::CHAINX_SYMBOL.to_vec())),
            510 + 25
        );

        assert_err!(
            TokenBalances::transfer(Some(a).into(), b.into(), Test::CHAINX_SYMBOL.to_vec(), 1000),
            "balance too low to send value"
        );
    })
}

#[test]
fn test_transfer_token() {
    with_externalities(&mut new_test_ext(), || {
        let a: u64 = 1; // accountid
        let b: u64 = 2; // accountid
        let btc_symbol = b"x-btc".to_vec();
        // issue 50 to account 1
        TokenBalances::issue(&a, &btc_symbol.clone(), 50).unwrap();
        // transfer
        TokenBalances::transfer(Some(a).into(), b.into(), btc_symbol.clone(), 25).unwrap();
        // sum not change
        assert_eq!(TokenBalances::total_free_token(&btc_symbol.clone()), 150);
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 25);
        assert_eq!(TokenBalances::free_token(&(b, btc_symbol.clone())), 25);
        assert_eq!(Balances::free_balance(&a), 990);

        assert_err!(
            TokenBalances::transfer(Some(a).into(), b.into(), btc_symbol.clone(), 50),
            "free token too low to send value"
        )
    })
}

#[test]
fn test_transfer_to_self() {
    with_externalities(&mut new_test_ext(), || {
        let a: u64 = 1; // accountid
        let btc_symbol = b"x-btc".to_vec();
        // issue 50 to account 1
        TokenBalances::issue(&a, &btc_symbol.clone(), 50).unwrap();
        // transfer
        assert_err!(
            TokenBalances::transfer(Some(a).into(), a.into(), btc_symbol.clone(), 25),
            "transactor and dest account are same"
        );

        // sum not change
        assert_eq!(TokenBalances::total_free_token(&btc_symbol.clone()), 150);
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 50);
        assert_eq!(Balances::free_balance(&a), 990);
    })
}

#[test]
fn test_transfer_err() {
    with_externalities(&mut new_test_ext(), || {
        let a: u64 = 1; // accountid
        let b: u64 = 2; // accountid
        let btc_symbol = b"x-btc".to_vec();
        // issue 50 to account 2
        TokenBalances::issue(&b, &btc_symbol.clone(), 50).unwrap();
        // transfer
        TokenBalances::transfer(Some(b).into(), a.into(), btc_symbol.clone(), 25).unwrap();
        // sum not change
        assert_eq!(TokenBalances::total_free_token(&btc_symbol.clone()), 150);
        assert_eq!(TokenBalances::free_token(&(b, btc_symbol.clone())), 25);
        assert_eq!(TokenBalances::total_token_of(&a, &btc_symbol.clone()), 25);
        assert_eq!(Balances::free_balance(&b), 500);

        assert_err!(
            TokenBalances::transfer(Some(b).into(), a.into(), btc_symbol.clone(), 1),
            "chainx balance is not enough after this tx, not allow to be killed at here"
        );
        assert_eq!(Balances::free_balance(&b), 500);
    })
}

#[test]
fn test_set_token() {
    with_externalities(&mut new_test_ext2(), || {
        let a: u64 = 1; // accountid
        let btc_symbol = b"x-btc".to_vec();
        TokenBalances::issue(&a, &btc_symbol.clone(), 50).unwrap();
        assert_ok!(TokenBalances::set_free_token(
            a.into(),
            Test::CHAINX_SYMBOL.to_vec(),
            500
        ));
        assert_eq!(Balances::free_balance(&a), 500);

        assert_ok!(TokenBalances::set_free_token(
            a.into(),
            btc_symbol.clone(),
            500
        ));
        assert_eq!(TokenBalances::free_token(&(a, btc_symbol.clone())), 500);
        assert_eq!(TokenBalances::total_token(&btc_symbol), 500 + 100);

        assert_ok!(TokenBalances::set_free_token(
            a.into(),
            btc_symbol.clone(),
            600
        ));
        assert_eq!(TokenBalances::free_token(&(a, btc_symbol.clone())), 600);
        assert_eq!(TokenBalances::total_token(&btc_symbol), 600 + 100);
    })
}

#[test]
fn test_char_valid() {
    with_externalities(&mut new_test_ext(), || {
        let to: balances::Address<Test> = balances::address::Address::Id(2);
        let origin = system::RawOrigin::Signed(1).into();
        let sym = b"".to_vec();
        assert_err!(
            TokenBalances::transfer(origin, to.clone(), sym, 10),
            "symbol length too long or zero"
        );

        let origin = system::RawOrigin::Signed(1).into();
        let sym = b"dfasdlfjkalsdjfklasjdflkasjdfklasjklfasjfkdlsajf".to_vec();
        assert_err!(
            TokenBalances::transfer(origin, to.clone(), sym, 10),
            "symbol length too long or zero"
        );

        let origin = system::RawOrigin::Signed(1).into();
        let sym = b"23jfkldae(".to_vec();
        assert_err!(
            TokenBalances::transfer(origin, to.clone(), sym, 10),
            "not a valid symbol char for number, capital/small letter or '-', '.', '|', '~'"
        );

        let t: Token = Token::new(b"x-btc2".to_vec(), b"btc token fdsfsdfasfasdfasdfasdfasdfasdfasdfjaskldfjalskdjflk;asjdfklasjkldfjalksdjfklasjflkdasjflkjkladsjfkrtewtewrtwertrjhjwretywertwertwerrtwerrtwerrtwertwelasjdfklsajdflkaj".to_vec(), 8);
        assert_err!(
            TokenBalances::register_token(t, 0, 0),
            "token desc length too long"
        );
        let t: Token = Token::new(b"x-btc?".to_vec(), b"btc token".to_vec(), 8);
        assert_err!(
            TokenBalances::register_token(t, 0, 0),
            "not a valid symbol char for number, capital/small letter or '-', '.', '|', '~'"
        )
    })
}

#[test]
fn test_chainx() {
    with_externalities(&mut new_test_ext2(), || {
        let a: u64 = 1; // accountid
        let sym = Test::CHAINX_SYMBOL.to_vec();
        assert_err!(
            TokenBalances::issue(&a, &sym, 100),
            "can't issue chainx token"
        );

        assert_ok!(TokenBalances::reserve(&a, &sym, 100, Default::default()));
        assert_eq!(Balances::free_balance(&a), 900);
        assert_eq!(Balances::reserved_balance(&a), 100);
        assert_eq!(
            TokenBalances::reserved_token(&(a, sym.clone(), Default::default())),
            100
        );

        assert_ok!(TokenBalances::unreserve(&a, &sym, 50, Default::default()));
        assert_eq!(Balances::free_balance(&a), 950);
        assert_eq!(
            TokenBalances::reserved_token(&(a, sym.clone(), Default::default())),
            50
        );
        assert_eq!(Balances::reserved_balance(&a), 50);
        assert_err!(
            TokenBalances::destroy(&a, &sym, 50, Default::default()),
            "can't destroy chainx token"
        );
    })
}

#[test]
fn test_chainx_err() {
    with_externalities(&mut new_test_ext2(), || {
        let a: u64 = 1; // accountid
        let sym = Test::CHAINX_SYMBOL.to_vec();

        assert_err!(
            TokenBalances::reserve(&a, &sym, 2000, Default::default()),
            "chainx free token too low to reserve"
        );
        assert_err!(
            TokenBalances::unreserve(&a, &sym, 10, Default::default()),
            "chainx reserved token too low to unreserve"
        );

        let i: i32 = -1;
        let larger_balance: TokenBalance = (i as u64) as u128 + 2;

        assert_eq!(larger_balance, 18446744073709551617);
        assert_eq!(larger_balance as u64, 1);

        assert_ok!(TokenBalances::reserve(
            &a,
            &sym,
            larger_balance,
            Default::default()
        ));
        assert_eq!(Balances::free_balance(&a), 999);

        let i: i32 = -1;
        let max_balance: TokenBalance = i as u128;
        assert_eq!(max_balance as u64, 18446744073709551615);
        assert_err!(
            TokenBalances::reserve(&a, &sym, max_balance, Default::default()),
            "chainx free token too low to reserve"
        );
    })
}

#[test]
fn test_move() {
    with_externalities(&mut new_test_ext2(), || {
        let a: u64 = 1; // accountid
        let b: u64 = 2; // accountid
        let sym = Test::CHAINX_SYMBOL.to_vec();
        assert_ok!(TokenBalances::move_free_token(&a, &b, &sym, 100));
        assert_err!(
            TokenBalances::move_free_token(&a, &b, &sym, 1000),
            TokenErr::NotEnough
        );
        assert_eq!(Balances::free_balance(&a), 900);
        assert_eq!(Balances::free_balance(&b), 510 + 100);

        let sym = b"x-btc".to_vec();
        assert_err!(
            TokenBalances::move_free_token(&a, &b, &sym, 100),
            TokenErr::InvalidToken
        );

        TokenBalances::issue(&a, &sym, 100).unwrap();
        assert_ok!(TokenBalances::move_free_token(&a, &b, &sym, 100));
        assert_err!(
            TokenBalances::move_free_token(&a, &b, &sym, 1000),
            TokenErr::NotEnough
        );

        assert_eq!(TokenBalances::free_token(&(a.clone(), sym.clone())), 0);
        assert_eq!(TokenBalances::free_token(&(b.clone(), sym.clone())), 100);
    })
}
