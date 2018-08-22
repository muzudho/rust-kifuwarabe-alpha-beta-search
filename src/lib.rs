/// 探索部だぜ☆（＾～＾）
/// アルファベータ探索で、さらに　ネガマックスだぜ☆（＾ｑ＾）
extern crate kifuwarabe_movement;
extern crate kifuwarabe_position;

use GAME_RECORD_WRAP;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use std::collections::HashSet;


fn empty_leaf_callback() -> (Movement, i16) {
    (Movement::new(), 0)
}

fn empty_makemove_callback(_cap: &KmSyurui) {
}

fn empty_unmakemove_callback(_cap: &KmSyurui) {
}

fn empty_pick_movements_callback(_max_depth: i16, _cur_depth: i16) -> HashSet<u64> {
    HashSet::new()
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
/// 探索を打ち切るなら真。
fn empty_compare_best_callback(_best_movement: &mut Movement, _alpha: &mut i16, _beta: i16, _movement: Movement, _child_evaluation: i16) -> bool {
    false
}

/// 探索オブジェクト。思考開始時に作成して使う。
pub struct AlphaBetaSearcher{
    /// 探索を打ち切るなら真。
    pub quittance: bool,
    pub leaf_callback: fn() -> (Movement, i16),
    pub makemove_callback: fn(&KmSyurui),
    pub unmakemove_callback: fn(&KmSyurui),
    pub pick_movements_callback: fn(max_depth: i16, cur_depth: i16) -> HashSet<u64>,

    /// 指し手の比較。
    ///
    /// 1. ベストな指し手。
    /// 2. alpha。より良い手があれば増える。
    /// 3. beta。
    /// 4. 今回比較する指し手。
    /// 5. 今回比較する指し手の評価値。
    ///
    /// 探索を打ち切るなら真。
    pub compare_best_callback: fn(&mut Movement, &mut i16, i16, Movement, i16) -> bool,
}

impl AlphaBetaSearcher{
    pub fn new()->AlphaBetaSearcher{
        AlphaBetaSearcher{
            quittance: false,
            leaf_callback: empty_leaf_callback,
            makemove_callback: empty_makemove_callback,
            unmakemove_callback: empty_unmakemove_callback,
            pick_movements_callback: empty_pick_movements_callback,
            compare_best_callback: empty_compare_best_callback,
        }
    }


    /// 探索。
    /// 
    /// * `max_depth` - 潜りたい深さ。
    /// * `cur_depth` - 現在の深さ。末端が 0。
    /// * `min_alpha` - 最低評価値。これより低い評価値は無視する。
    /// * `beta` - 上限評価値。これより評価が高いなら探索を打ち切る。
    /// Returns: ベストムーブ, 評価値。
    pub fn search(&mut self, max_depth: i16, cur_depth: i16, min_alpha: i16, beta: i16) -> (Movement, i16) {

        if 0 == cur_depth {
            // 葉。
            return (self.leaf_callback)();
        }


        // 現局面の合法手を取得する。
        let hashset_movement = (self.pick_movements_callback)(max_depth, cur_depth);


        let mut best_movement = Movement::new();
        let mut alpha = min_alpha; // ベスト評価値
        'idea: for hash_mv in hashset_movement.iter() {
            let movement = Movement::from_hash( *hash_mv );

            if self.quittance {
                // 指す前に、探索を打ち切る。
                break;
            }

            // 1手指す。
            {
                GAME_RECORD_WRAP.try_write().unwrap().make_movement2(&movement, self.makemove_callback);
            }

            // 子を探索へ。
            let (_child_movement, mut child_evaluation) = self.search(max_depth, cur_depth-1, -beta, -alpha);
            // 相手の評価値を逆さにする。
            child_evaluation = -child_evaluation;

            // 比較して、一番良い手を選ぶ。
            let mut cutoff = false;
            if (self.compare_best_callback)(&mut best_movement, &mut alpha, beta, movement, child_evaluation)
            {
                // 手を戻したあと、探索を打ち切る。
                cutoff = true;
            }

            // 1手戻す。
            {
                GAME_RECORD_WRAP.try_write().unwrap().unmake_movement2(self.unmakemove_callback);
            }

            if cutoff || self.quittance {
                // 指した駒を戻したところで、探索を打ち切る。
                break;
            }
        }

        // 返却。
        (best_movement, alpha)
    }
}
