/// ```
/// ### 以下のコマンドで実行。 
/// cargo run --example main
/// ```
extern crate kifuwarabe_alpha_beta_search;
extern crate kifuwarabe_movement;
extern crate kifuwarabe_position;

use kifuwarabe_alpha_beta_search::*;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use std::collections::HashSet;


/// 任意の構造体を作成する。
struct Searcher {

}
impl Searcher {
    fn new() -> Searcher {
        Searcher {

        }
    }
}




fn leaf_callback<T>(_t: &mut T) -> (Movement, i16)
{
    println!("- 末端局面を評価する。");
    (Movement::new(), 0)
}

fn makemove_callback(_cap: &KmSyurui) {
    println!("- 1手指す。");
}

fn unmakemove_callback(_cap: &KmSyurui) {
    println!("- 1手戻す。");
}

/// # Returns.
///
/// 1. 指し手のハッシュのセット。
/// 2. 現在の探索を放棄し、すみやかに安全に終了するなら真。
fn pick_movements_callback<T>(_t: &mut T, _max_depth: i16, _cur_depth: i16) -> (HashSet<u64>, bool)
{
    println!("- 選択肢を返す。");
    (HashSet::new(), false)
}

/// 指し手の比較。
///
/// # Arguments.
///
/// * `_best_movement` - ベストな指し手。
/// * `_alpha` - alpha。より良い手があれば増える。
/// * `_beta` - beta。
/// * `_movement` - 今回比較する指し手。
/// * `_child_evaluation` - 今回比較する指し手の評価値。
///
/// # Returns.
///
/// 1. 探索を打ち切るなら真。（ベータカット）
/// 2. 現在の探索を放棄し、すみやかに安全に終了するなら真。
fn compare_best_callback<T>(_t: &mut T, _best_movement: &mut Movement, _alpha: &mut i16, _beta: i16, _movement: Movement, _child_evaluation: i16) -> (bool, bool)
{
    println!("- 手を比較し、より良い方を選ぶ。");
    (false, false)
}




fn main() {

    // 任意の構造体を作成する。
    let mut searcher = Searcher::new();

    // 任意の構造体を受け取る、コールバック カタログを作成する。
    let mut callback_catalog = CallbackCatalog {
        leaf_callback: leaf_callback,
        makemove_callback: makemove_callback,
        unmakemove_callback: unmakemove_callback,
        pick_movements_callback: pick_movements_callback,
        compare_best_callback: compare_best_callback,
    };

    let max_depth = 1;
    let cur_depth = 1;
    let min_alpha = -<i16>::max_value(); // <i16>::min_value() (負値) にすると、負数の方が変域が1だけ広く、正負符号を反転したときに正数があふれてしまうので、正の最大数に - を付ける。
    let beta = <i16>::max_value();
    let (_best_movement, _evaluation) = search(&mut searcher, &mut callback_catalog, max_depth, cur_depth, min_alpha, beta);

}