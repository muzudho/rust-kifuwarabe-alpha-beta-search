/// ```
/// ### 以下のコマンドで実行。 
/// cargo run --example main
/// ```
extern crate kifuwarabe_alpha_beta_search;

use kifuwarabe_alpha_beta_search::*;
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




fn visit_leaf_callback<T>(_t: &mut T, display_information: &DisplayInformation) -> (i16)
{
    println!("- 末端局面を評価する。 nodes: {}", display_information.nodes);
    0
}

fn makemove_callback<T>(_t: &mut T, movement_hash: u64) {
    println!("- 1手指す。 hash: {}", movement_hash);
}

fn unmakemove_callback<T>(_t: &mut T) {
    println!("- 1手戻す。");
}

/// # Returns.
///
/// 1. 指し手のハッシュのセット。
/// 2. 現在の探索を放棄し、すみやかに安全に終了するなら真。
fn pick_movements_callback<T>(_t: &mut T, _max_depth: i16, _cur_depth: i16) -> (HashSet<u64>, bool)
{
    println!("- 選択肢を返す。");
    let mut hashset = HashSet::<u64>::new();
    hashset.insert(0);
    hashset.insert(1);
    hashset.insert(2);
    (hashset, false)
}

/// 指し手の比較。
///
/// # Arguments.
///
/// * `best_movement_hash` - ベストな指し手のハッシュ値。
/// * `_alpha` - alpha。より良い手があれば増える。
/// * `_beta` - beta。
/// * `_movement` - 今回比較する指し手のハッシュ値。
/// * `_child_evaluation` - 今回比較する指し手の評価値。
///
/// # Returns.
///
/// 1. 探索を打ち切るなら真。（ベータカット）
/// 2. 現在の探索を放棄し、すみやかに安全に終了するなら真。
fn compare_best_callback<T>(_t: &mut T, _best_movement_hash: &mut u64, _alpha: &mut i16, _beta: i16, _movement: u64, _child_evaluation: i16) -> (bool, bool)
{
    println!("- 手を比較し、より良い方を選ぶ。");
    (false, false)
}




fn main() {

    // 任意の構造体を作成する。
    let mut searcher = Searcher::new();

    // 任意の構造体を受け取る、コールバック カタログを作成する。
    let mut callback_catalog = CallbackCatalog {
        visit_leaf_callback: visit_leaf_callback,
        makemove_callback: makemove_callback,
        unmakemove_callback: unmakemove_callback,
        pick_movements_callback: pick_movements_callback,
        compare_best_callback: compare_best_callback,
    };

    let max_depth = 2;
    let cur_depth = max_depth;
    let min_alpha = -<i16>::max_value(); // <i16>::min_value() (負値) にすると、負数の方が変域が1だけ広く、正負符号を反転したときに正数があふれてしまうので、正の最大数に - を付ける。
    let beta = <i16>::max_value();

    let (_best_movement, _evaluation) = start(&mut searcher, &mut callback_catalog, max_depth, cur_depth, min_alpha, beta);

}