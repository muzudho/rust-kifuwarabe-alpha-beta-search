/// 探索部だぜ☆（＾～＾）
/// アルファベータ探索で、さらに　ネガマックスだぜ☆（＾ｑ＾）
extern crate kifuwarabe_movement;
extern crate kifuwarabe_position;

use GAME_RECORD_WRAP;
use kifuwarabe_movement::*;
use kifuwarabe_position::*;
use std::collections::HashSet;

/// コールバック関数を差し替えられる形にしたオブジェクト。
///
/// # Generic types.
///
/// * `T` - 任意のオブジェクト。search関数の引数に渡したオブジェクトが、コールバック関数の引数に渡される。
pub struct CallbackCatalog<T> {

    /// 末端局面評価時。
    ///
    /// # Arguments.
    ///
    /// * `t` - 任意のオブジェクト。
    ///
    /// # Returns.
    ///
    /// 0. 指し手
    /// 1. 評価値
    pub leaf_callback: fn(t: &mut T) -> (Movement, i16),

    /// １手指す。
    ///
    /// # Arguments.
    ///
    /// * `&KmSyurui` - 駒種類。
    pub makemove_callback: fn(&KmSyurui),

    /// １手戻す。
    ///
    /// # Arguments.
    ///
    /// * `&KmSyurui` - 駒種類。
    pub unmakemove_callback: fn(&KmSyurui),

    /// 指し手生成。
    ///
    /// # Arguments.
    ///
    /// * `t` - 任意のオブジェクト。
    /// * `max_depth` - 探索の最大深さ。
    /// * `cur_depth` - 現在探索中の深さ。
    ///
    /// # Returns.
    ///
    /// 1. 指し手のハッシュのセット。
    /// 2. 探索をすべて打ち切るなら真。
    pub pick_movements_callback: fn(t: &mut T, max_depth: i16, cur_depth: i16) -> (HashSet<u64>, bool),

    /// 指し手の比較。
    ///
    /// # Arguments.
    ///
    /// * `t` - 任意のオブジェクト。
    /// * `bestmove` - ベストな指し手。
    /// * `alpha` - より良い手があれば増える。
    /// * `beta` - ベータ。
    /// * `movement` - 今回比較する指し手のハッシュ値。
    /// * `evaluation` - 今回比較する指し手の評価値。
    ///
    /// # Returns.
    ///
    /// 1. 探索を打ち切るなら真。（ベータカット）
    /// 2. 探索をすべて打ち切るなら真。
    pub compare_best_callback: fn(t: &mut T, bestmove: &mut Movement, alpha: &mut i16, beta: i16, movement: Movement, evaluation: i16) -> (bool, bool),
}

/// 探索。
/// 
/// # Arguments.
///
/// * `max_depth` - 潜りたい深さ。
/// * `cur_depth` - 現在の深さ。末端が 0。
/// * `min_alpha` - 最低評価値。これより低い評価値は無視する。
/// * `beta` - 上限評価値。これより評価が高いなら探索を打ち切る。
///
/// # Returns.
///
/// 0. ベストムーブ。
/// 1. 評価値。
pub fn search<T>(t: &mut T, callback_catalog: &mut CallbackCatalog<T>, max_depth: i16, cur_depth: i16, min_alpha: i16, beta: i16) -> (Movement, i16)
{

    if 0 == cur_depth {
        // 葉。
        return (callback_catalog.leaf_callback)(t);
    }


    // 現局面の合法手を取得する。
    let (hashset_movement, quittance1) = (callback_catalog.pick_movements_callback)(t, max_depth, cur_depth);
    if quittance1 {
        // 指す前に、すべての探索を打ち切る。
        return (Movement::new(), min_alpha);
    }


    let mut best_movement = Movement::new();
    let mut alpha = min_alpha; // ベスト評価値
    'idea: for hash_mv in hashset_movement.iter() {
        let movement = Movement::from_hash( *hash_mv );

        // 1手指す。
        {
            GAME_RECORD_WRAP.try_write().unwrap().make_movement2(&movement, callback_catalog.makemove_callback);
        }

        // 子を探索へ。
        let (_child_movement, mut child_evaluation) = search(t, callback_catalog, max_depth, cur_depth-1, -beta, -alpha);
        // 相手の評価値を逆さにする。
        child_evaluation = -child_evaluation;

        // 比較して、一番良い手を選ぶ。
        let mut cutoff = false;
        let (beta_cutoff, quittance2) = (callback_catalog.compare_best_callback)(t, &mut best_movement, &mut alpha, beta, movement, child_evaluation);
        if beta_cutoff
        {
            // 手を戻したあと、探索を打ち切る。
            cutoff = true;
        }

        // 1手戻す。
        {
            GAME_RECORD_WRAP.try_write().unwrap().unmake_movement2(callback_catalog.unmakemove_callback);
        }

        if cutoff || quittance2 {
            // 指した駒を戻したところで、探索を打ち切る。
            break;
        }
    }

    // 返却。
    (best_movement, alpha)
}
