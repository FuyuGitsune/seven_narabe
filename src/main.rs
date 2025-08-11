use std::io::stdin;
use owo_colors::OwoColorize;
use rand::seq::SliceRandom;

fn main() {
    let mut break_flag = 5;
    let mut break_flag_p = 0;
    let mut win_num = 0;
    let mut lose_num = 0;
    let mut lock = false;
    let  lock_num = "27182818";

    println!("----------7並べ(vs computer)----------");
    lb();
    println!("--Get ready?（ENTERで続行）--");
    let f_lock = input_str();
    if f_lock == "5" || f_lock == "l" || f_lock == "L" || f_lock == "lock"{
        lock = true;
    }
    lb();
    if game_main(){
        win_num += 1;
        break_flag = 0;
    }else{
        lose_num += 1;
        break_flag += 1;
    }
    lb();
    println!(" ------result------");
    println!("     win  : {win_num}");
    println!("     lose : {lose_num}");
    println!("     rate : {}", win_num*100);
    lb();

    loop{
        println!("もう一度しますか？Y(1)/N(0)");
        let retry = input_str();
        if retry == "y" || retry == "Y" || retry == "1"{
            lb();
            if game_main(){
                win_num += 1;
                if break_flag <= 3{
                    break_flag_p = 1;
                    break_flag = 5;
                }else{
                    break_flag = 0;
                }
            }else{
                lose_num += 1;
                break_flag += 1;
            }
            lb();
            println!(" ------result------");
            let win_rate:f64 = (win_num as f64 / (win_num as f64 + lose_num as f64) * 1000.0).round() / 10.0;
            println!("     win  : {}", win_num);
            println!("     lose : {}", lose_num);
            println!("     rate : {}", win_rate);
            lb();
            if break_flag_p == 1{
                println!("任意の連続5戦中2回勝利で表示されるメッセージです。おめでとう。");
                lb();
                break_flag_p = 0;
            }
        }else if retry == "n" || retry == "N" || retry == "0"{
            if lock{
                println!("プログラムの終了はパスワードによって保護されています。パスワードを入力してください。");
                if input_str() == lock_num{
                    println!("認証されました。終了します。");
                    break;
                }else{
                    println!("パスワードが間違っています。");
                }
            }else{
                break;
            }
        }else if retry == "L" || retry == "l" || retry == "lock" || retry == "Lock" || retry == "5"{
            println!("この操作はパスワード認証が必要です。");
            if input_str() == lock_num{
                lock = !lock;
                if lock{
                    println!("終了無許可状態に変更");
                }else{
                    println!("終了無許可状態を解除");
                }
            }else{
                println!("パスワードが間違っています。");
            }
        }
    }
}

fn input_int()->f64{
    loop{
    let mut a = String::new();
    stdin().read_line(&mut a).expect("Failed to read line");
    let a: f64 = match a.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };
    if a < 0.0{
        continue;
    }
    return a
    }
}

fn input_str()->String{
    let mut a = String::new();
    stdin().read_line(&mut a).unwrap();
    return a.trim().parse().unwrap();
}

fn lb(){
    println!(" ");
}

#[allow(unused_assignments)]
fn game_main()->bool{
    //準備開始

    let mut pl_c = Vec::new(); //pl->mピーエルです！！
    let mut c1_c = Vec::new();
    let mut c2_c = Vec::new();
    let mut c3_c = Vec::new();

    let mut stage = Vec::new();

    let mut trump = Vec::new();
    for n in 0..52{
        trump.push(n);
    }

    let mut rng = rand::thread_rng();
    trump.shuffle(&mut rng);

    let mut turn = 0;

    #[allow(clippy::needless_range_loop)]
    for n in 0..52{
        if trump[n] % 13 != 6{
            if n / 13 < 1{
                pl_c.push(trump[n]);
            }else if n / 13 < 2{
                c1_c.push(trump[n]);
            }else if n / 13 < 3{
                c2_c.push(trump[n]);
            }else if n / 13 < 4{
                c3_c.push(trump[n]);
            }
        }else{
            if trump[n] == 6{
                turn = n / 13;
            }
            stage.push(trump[n]);
        }
    }

    pl_c.sort();
    c1_c.sort();
    c2_c.sort();
    c3_c.sort();
    stage.sort();

    let mut pl_save = true;
    let mut c1_save = true;
    let mut c2_save = true;
    let mut c3_save = true;

    let mut pl_pass = 3;
    let mut c1_pass = 3;
    let mut c2_pass = 3;
    let mut c3_pass = 3;

    let mut c1_r = 300;
    let mut c2_r = 300;
    let mut c3_r = 300;
    let mut pl_r = 300;

    let mut winner = "";
    
    draw_stage(stage.clone(), &pl_pass, &c1_pass, &c2_pass, &c3_pass, pl_c.len(), c1_c.len(), c2_c.len(), c3_c.len(), &pl_save, &c1_save, &c2_save, &c3_save);



    //カード配り終了か？
    loop{

        if turn == 0{
            draw_stage(stage.clone(), &pl_pass, &c1_pass, &c2_pass, &c3_pass, pl_c.len(), c1_c.len(), c2_c.len(), c3_c.len(), &pl_save, &c1_save, &c2_save, &c3_save);
            
            if pl_save{//
                pl_r = player_move(pl_c.clone(), pl_pass, stage.clone());
                if pl_r == 200{
                    pl_save = false;
                    for i in pl_c.clone(){
                        stage.push(i);
                    }
                    println!("player -> dropout.");
                }else if pl_r == 100{
                    pl_pass -= 1;
                    println!("player -> pass.");
                }else{
                    stage.push(pl_r);
                    pl_c.retain(|&x| x != pl_r);
                    if pl_r / 13 < 1{
                        println!("player -> D{}", (pl_r % 13)+1);
                    }else if pl_r / 13 < 2{
                        println!("player -> C{}", (pl_r % 13)+1);
                    }else if pl_r / 13 < 3{
                        println!("player -> H{}", (pl_r % 13)+1);
                    }else if pl_r / 13 < 4{
                        println!("player -> S{}", (pl_r % 13)+1);
                    }
                }
            }else{
                println!("player -> Already dropped out.");
            }
            turn = 1;
        }else if turn == 1{
            if c1_save{
                c1_r = computer_move(c1_c.clone(), c1_pass, stage.clone());
                if c1_r == 200{
                    c1_save = false;
                    for i in c1_c.clone(){
                        stage.push(i);
                    }
                    println!("computer1 -> dropout.");
                }else if c1_r == 100{
                    c1_pass -= 1;
                    println!("computer1 -> pass.");
                }else{
                    stage.push(c1_r);
                    c1_c.retain(|&x| x != c1_r);
                    if c1_r / 13 < 1{
                        println!("computer1 -> D{}", (c1_r % 13)+1);
                    }else if c1_r / 13 < 2{
                        println!("computer1 -> C{}", (c1_r % 13)+1);
                    }else if c1_r / 13 < 3{
                        println!("computer1 -> H{}", (c1_r % 13)+1);
                    }else if c1_r / 13 < 4{
                        println!("computer1 -> S{}", (c1_r % 13)+1);
                    }
                }
            }else{
                println!("computer1 -> Already dropped out.");
            }
            turn = 2;
        }else if turn == 2{
            if c2_save{
                c2_r = computer_move(c2_c.clone(), c2_pass, stage.clone());
                if c2_r == 200{
                    c2_save = false;
                    for i in c2_c.clone(){
                        stage.push(i);
                    }
                    println!("computer2 -> dropout.");
                }else if c2_r == 100{
                    c2_pass -= 1;
                    println!("computer2 -> pass.");
                }else{
                    stage.push(c2_r);
                    c2_c.retain(|&x| x != c2_r);
                    if c2_r / 13 < 1{
                        println!("computer2 -> D{}", (c2_r % 13)+1);
                    }else if c2_r / 13 < 2{
                        println!("computer2 -> C{}", (c2_r % 13)+1);
                    }else if c2_r / 13 < 3{
                        println!("computer2 -> H{}", (c2_r % 13)+1);
                    }else if c2_r / 13 < 4{
                        println!("computer2 -> S{}", (c2_r % 13)+1);
                    }
                }
            }else{
                println!("computer2 -> Already dropped out.");
            }
            turn = 3;
        }else if turn == 3{
            if c3_save{
                c3_r = computer_move(c3_c.clone(), c3_pass, stage.clone());
                if c3_r == 200{
                    c3_save = false;
                    for i in c3_c.clone(){
                        stage.push(i);
                    }
                    println!("computer3 -> dropout.");
                }else if c3_r == 100{
                    c3_pass -= 1;
                    println!("computer3 -> pass.");
                }else{
                    stage.push(c3_r);
                    c3_c.retain(|&x| x != c3_r);
                    if c3_r / 13 < 1{
                        println!("computer3 -> D{}", (c3_r % 13)+1);
                    }else if c3_r / 13 < 2{
                        println!("computer3 -> C{}", (c3_r % 13)+1);
                    }else if c3_r / 13 < 3{
                        println!("computer3 -> H{}", (c3_r % 13)+1);
                    }else if c3_r / 13 < 4{
                        println!("computer3 -> S{}", (c3_r % 13)+1);
                    }
                }
            }else{
                println!("computer3 -> Already dropped out.");
            }
            turn = 0;
        }
        stage.sort();
        let mut save_num = 0;
        if pl_save{
            save_num += 1;
            winner = "player";
        }
        if c1_save{
            save_num += 1;
            winner = "computer1";
        }
        if c2_save{
            save_num += 1;
            winner = "computer2";
        }
        if c3_save{
            save_num += 1;
            winner = "computer3";
        }
        if save_num == 1{
            break;
        }
        if pl_c.is_empty(){
            winner = "player";
            break;
        }else if c1_c.is_empty(){
            winner = "computer1";
            break;
        }else if c2_c.is_empty(){
            winner = "computer2";
            break;
        }else if c3_c.is_empty(){
            winner = "computer3";
            break;
        }

    }
    
    draw_stage(stage.clone(), &pl_pass, &c1_pass, &c2_pass, &c3_pass, pl_c.len(), c1_c.len(), c2_c.len(), c3_c.len(), &pl_save, &c1_save, &c2_save, &c3_save);
    
    println!("Game set!! winner is -> {}", winner);
    winner == "player"
}

#[allow(clippy::too_many_arguments)]
fn draw_stage(stage :Vec<i32>, pl_pass :&i32, c1_pass :&i32, c2_pass :&i32, c3_pass :&i32, pl_cl :usize, c1_cl :usize, c2_cl :usize, c3_cl :usize, pl_save :&bool, c1_save :&bool, c2_save :&bool, c3_save :&bool){ 
    println!(" ");
    //盤面作り
    let ok = "o ";
    let ng = "- ";
    let mut d = String::from("D ");
    let mut c = String::from("C ");
    let mut h = String::from("H ");
    let mut s = String::from("S ");

    for i in 0..52{
        if !stage.iter().any(|&x| x == i){
            if i / 13 < 1{
                d.push_str(ng);
            }else if i / 13 < 2{
                c.push_str(ng);
            }else if i / 13 < 3{
                h.push_str(ng);
            }else{
                s.push_str(ng);
            }
        }else if i /13 < 1{
            d.push_str(ok);
        }else if i / 13 < 2{
            c.push_str(ok);
        }else if i / 13 < 3{
            h.push_str(ok);
        }else{
            s.push_str(ok);
        }
    }


    println!("{}    -------------------info-------------------", "   A 2 3 4 5 6 7 8 9 T J Q K  ".black().on_white());
    if *pl_save{
        println!("{}{}{}       player : Survival, pass = {}, {} left.", " ".red().on_white(), d.red().on_white(), " ".red().on_white(), pl_pass, pl_cl);
    }else{
        println!("{}{}{}       player : dropout.", " ".red().on_white(), d.red().on_white(), " ".red().on_white());
    }
    if *c1_save{
        println!("{}{}{}    computer1 : Survival, pass = {}, {} left.", " ".cyan().on_white(), c.cyan().on_white(), " ".cyan().on_white(), c1_pass, c1_cl);
    }else{
        println!("{}{}{}    computer1 : dropout.", " ".cyan().on_white(), c.cyan().on_white(), " ".cyan().on_white());
    }
    if *c2_save{
        println!("{}{}{}    computer2 : Survival, pass = {}, {} left.", " ".purple().on_white(), h.purple().on_white(), " ".purple().on_white(), c2_pass, c2_cl);
    }else{
        println!("{}{}{}    computer2 : dropout.", " ".purple().on_white(), h.purple().on_white(), " ".purple().on_white());
    }
    if *c3_save{
        println!("{}{}{}    computer3 : Survival, pass = {}, {} left.", " ".blue().on_white(), s.blue().on_white(), " ".blue().on_white(), c3_pass, c3_cl);
    }else{
        println!("{}{}{}    computer3 : dropout.", " ".blue().on_white(), s.blue().on_white(), " ".blue().on_white());
    }
    println!(" ");
}

fn computer_move(comp_c :Vec<i32>, comp_pass :i32, stage :Vec<i32>)->i32{
    //comp_num -> アカウント、comp_c -> カード（ベクター）、comp_pass -> pass残り
    //returnは0~51の時はそれ。100ならパス。200なら脱落。

    //!stage.iter().any(|&x| x == i)
    //自分の所持未所持にかかわらず、出せる手を検索。
    let mut card_p = Vec::new();
    for i in 1..7{
        if !stage.iter().any(|&x| x == 6 - i){
            card_p.push(6-i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == i + 6){
            card_p.push(6+i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == 19 - i){
            card_p.push(19 - i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == 19 + i){
            card_p.push(19+i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == 32 - i){
            card_p.push(32- i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == i + 32){
            card_p.push(i + 32);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == 45 - i){
            card_p.push(45- i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == i + 45){
            card_p.push(45 + i);
            break;
        }
    }
    
    //println!("{:?}", card_p);

    //出せるカードを判定した。次はそれが自分が持つカードに含まれるか検証する。
    let mut comp_c_p = Vec::new();
    for i in card_p.clone(){
        if comp_c.iter().any(|&x| x == i){
            comp_c_p.push(i);
        }
    }
    //println!("{:?}", comp_c_p);
    //自分が出せるカードが分かった。
    if comp_c_p.is_empty(){
        if comp_pass == 0{
            return 200;
        }
        100
    }else{
        //脱落・パスの場合、値を返した。それ以外の場合についての処理を行う。
        let mut eval = Vec::new();
        for i in comp_c_p.clone(){
            //それぞれの値に対して評価を行う。まずカードの位置を出す。
            if i % 13 < 6{
                //左側の時、
                let mut search = i - 1;
                let mut space = 1;
                let mut matagi = 0;
                let mut matagi_f = true;

                while search % 13 != 12 && search != -1{
                    if !stage.iter().any(|&x| x == search){
                        space += 1;
                        if !comp_c.iter().any(|&x| x == search) && matagi_f{
                            matagi += 1;
                        }else{
                            matagi_f = false;
                        }
                    }
                    search -= 1;
                }
                if matagi_f{
                    matagi = 10;
                }
                //評価。
                if space == 1{
                    eval.push(1);
                }else if matagi == 0{
                    eval.push(2);
                }else if matagi == 1{
                    eval.push(3);
                }else if space <= matagi{
                    if space != 6{
                        eval.push(space * 2 + 1);
                    }else{
                        eval.push(12);
                    }
                }else{
                    eval.push(matagi * 2 + 2);
                }

            }else{
                //右側の時、
                let mut search = i + 1;
                let mut space = 1;
                let mut matagi = 0;
                let mut matagi_f = true;

                while search % 13 != 0{
                    if !stage.iter().any(|&x| x == search){
                        space += 1;
                        if !comp_c.iter().any(|&x| x == search) && matagi_f{
                            matagi += 1;
                        }else{
                            matagi_f = false;
                        }
                    }
                    search += 1;
                }
                if matagi_f{
                    matagi = 10;
                }
                //評価。
                if space == 1{
                    eval.push(1);
                }else if matagi == 0{
                    eval.push(2);
                }else if matagi == 1{
                    eval.push(3);
                }else if space <= matagi{
                    if space != 6{
                        eval.push(space * 2 + 1);
                    }else{
                        eval.push(12);
                    }
                }else{
                    eval.push(matagi * 2 + 2);
                }
            }
        }
        //最終判断
        let lank = eval.iter().min().unwrap();
        if *lank < 4 || comp_pass == 0{
            let index = eval.iter().position(|&x| x == *lank).unwrap();
            comp_c_p[index]
        }else{
            100
        }
    }
}

#[allow(unused_assignments)]
fn player_move(pl_c :Vec<i32>, pl_pass :i32, stage :Vec<i32>)->i32{
    let mut show = String::from("| ");
    for i in pl_c.clone(){
        if i / 13 < 1{
            show.push_str("D.");
        }else if i / 13 < 2{
            show.push_str("C.");
        }else if i / 13 < 3{
            show.push_str("H.");
        }else if i / 13 < 4{
            show.push_str("S.");
        }
        let stri: String = (i % 13 + 1).to_string();
        if stri == "1"{
            show.push('A');
        }else if stri == "10"{
            show.push('T');
        }else if stri == "11"{
            show.push('J');
        }else if stri == "12"{
            show.push('Q');
        }else if stri == "13"{
            show.push('K');
        }else{
            show.push_str(&stri);
        }
        show.push_str(" | ");
    }
    println!("Your card -> {}", show);
    lb();
    //comp_num -> アカウント、comp_c -> カード（ベクター）、comp_pass -> pass残り
    //returnは0~51の時はそれ。100ならパス。200なら脱落。

    //自分の所持未所持にかかわらず、出せる手を検索。
    let mut card_p = Vec::new();
    for i in 1..7{
        if !stage.iter().any(|&x| x == 6 - i){
            card_p.push(6-i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == i + 6){
            card_p.push(6+i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == 19 - i){
            card_p.push(19 - i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == 19 + i){
            card_p.push(19+i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == 32 - i){
            card_p.push(32- i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == i + 32){
            card_p.push(i + 32);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == 45 - i){
            card_p.push(45- i);
            break;
        }
    }
    for i in 1..7{
        if !stage.iter().any(|&x| x == i + 45){
            card_p.push(45 + i);
            break;
        }
    }
    
    //println!("{:?}", card_p);

    //出せるカードを判定した。次はそれが自分が持つカードに含まれるか検証する。
    let mut pl_c_p = Vec::new();
    for i in card_p.clone(){
        if pl_c.iter().any(|&x| x == i){
            pl_c_p.push(i);
        }
    }
    //println!("{:?}", comp_c_p);
    //自分が出せるカードが分かった。
    if pl_c_p.is_empty(){
        if pl_pass == 0{
            println!("貴方には脱落しか方法はありません。(ENTERで続行)");
            input_str();
            return 200;
        }
        println!("貴方にはパスという手段しかありません。(ENTERで続行)");
        input_str();
        return 100;
    }
    println!("---貴方ができる行動一覧---");
    for i in 0..=pl_c_p.len(){
        if i != pl_c_p.len(){
            if pl_c_p[i] / 13 < 1{
                println!(" {} : D.{}", i, pl_c_p[i] % 13 + 1);
            }else if pl_c_p[i] / 13 < 2{
                println!(" {} : C.{}", i, pl_c_p[i] % 13 + 1);
            }else if pl_c_p[i] / 13 < 3{
                println!(" {} : H.{}", i, pl_c_p[i] % 13 + 1);
            }else if pl_c_p[i] / 13 < 4{
                println!(" {} : S.{}", i, pl_c_p[i] % 13 + 1);
            }
        }else if pl_pass == 0{
                println!(" {} : 脱落", i);
        }else{
                println!(" {} : パス", i);
        }
    }

    lb();
    println!("以上の中から数字で何を出すか入力してください。");
    let mut pl_re = 300;
    loop{
        pl_re = input_int() as usize;
        if pl_re <= pl_c_p.len(){
            break;
        }
        println!("有効な数字を入力してください");
    }
    if pl_re == pl_c_p.len(){
        if pl_pass == 0{
            return 200;
        }else{
            return 100;
        }
    }
    pl_c_p[pl_re]
}
