/// 探索部だぜ☆（＾～＾）
/// アルファベータ探索で、さらに　ネガマックスだぜ☆（＾ｑ＾）
use std::collections::HashSet;

/// 投了。
pub const RESIGN_HASH : u64 = 0;

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
    /// 0. 評価値
    pub visit_leaf_callback: fn(t: &mut T, display_information: &DisplayInformation) -> (i16),

    /// １手指す。
    ///
    /// # Arguments.
    ///
    /// * `movement_hash` - 指し手のハッシュ値。
    pub makemove_callback: fn(t: &mut T, movement_hash: u64),

    /// １手戻す。
    pub unmakemove_callback: fn(t: &mut T),

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
    /// 2. 探索をすみやかに安全に終了するなら真。
    pub pick_movements_callback: fn(t: &mut T, max_depth: i16, cur_depth: i16) -> (HashSet<u64>, bool),

    /// 指し手の比較。
    ///
    /// # Arguments.
    ///
    /// * `t` - 任意のオブジェクト。
    /// * `best_movement_hash` - ベストな指し手のハッシュ値。
    /// * `alpha` - より良い手があれば増える。
    /// * `beta` - ベータ。
    /// * `movement_hash` - 今回比較する指し手のハッシュ値。
    /// * `evaluation` - 今回比較する指し手の評価値。
    ///
    /// # Returns.
    ///
    /// 1. 探索を打ち切るなら真。（ベータカット）
    /// 2. 探索をすみやかに安全に終了するなら真。
    pub compare_best_callback: fn(t: &mut T, best_movement_hash: &mut u64, alpha: &mut i16, beta: i16, movement_hash: u64, evaluation: i16) -> (bool, bool),
}

/// 情報表示
pub struct DisplayInformation {
    // 探索ノード数。1手戻したところで加算。
    pub nodes: i64,
}
impl DisplayInformation {
    pub fn new() -> DisplayInformation {
        DisplayInformation {
            nodes: 0,
        }
    }
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
/// 0. 最善手のハッシュ値。
/// 1. 評価値。
pub fn start<T>(t: &mut T, callback_catalog: &mut CallbackCatalog<T>, max_depth: i16, cur_depth: i16, min_alpha: i16, beta: i16) -> (u64, i16)
{
    let mut display_information = DisplayInformation::new();
    search(t, callback_catalog, max_depth, cur_depth, min_alpha, beta, &mut display_information)
}


/// 探索。
/// 
/// # Arguments.
///
/// * `max_depth` - 潜りたい深さ。
/// * `cur_depth` - 現在の深さ。末端が 0。
/// * `min_alpha` - 最低評価値。これより低い評価値は無視する。
/// * `beta` - 上限評価値。これより評価が高いなら探索を打ち切る。
/// * `display_information` - 画面表示情報。
///
/// # Returns.
///
/// 0. 最善手のハッシュ値。
/// 1. 評価値。
fn search<T>(t: &mut T, callback_catalog: &mut CallbackCatalog<T>, max_depth: i16, cur_depth: i16, min_alpha: i16, beta: i16, display_information: &mut DisplayInformation) -> (u64, i16)
{
    // 現局面の合法手を取得する。
    let (hashset_movement, quittance1) = (callback_catalog.pick_movements_callback)(t, max_depth, cur_depth);
    if quittance1 {
        // 指し手生成が中断された。探索をすみやかに安全に終了する。
        return (RESIGN_HASH, min_alpha);
    }


    let mut best_movement_hash = RESIGN_HASH; // 手が無かったら投了
    let mut alpha = min_alpha; // ベスト評価値
    'idea: for next_movement_hash in hashset_movement.iter() {

        // 1手指す。
        (callback_catalog.makemove_callback)(t, *next_movement_hash);

        let mut child_evaluation;
        if 0 == cur_depth-1 {
            // 葉。
            child_evaluation = (callback_catalog.visit_leaf_callback)(t, display_information);

        } else {
            // 子を探索へ。
            let (_child_movement_hash, opponent_evaluation) = search(t, callback_catalog, max_depth, cur_depth-1, -beta, -alpha, display_information);
            // 相手の評価値を逆さにする。
            child_evaluation = -opponent_evaluation;

        }

        // 比較して、一番良い手を選ぶ。
        let mut cutoff = false;
        let (beta_cutoff, quittance2) = (callback_catalog.compare_best_callback)(t, &mut best_movement_hash, &mut alpha, beta, *next_movement_hash, child_evaluation);
        if beta_cutoff
        {
            // 手を戻したあと、探索を打ち切る。
            cutoff = true;
        }

        // 1手戻す。
        (callback_catalog.unmakemove_callback)(t);
        display_information.nodes += 1;

        if cutoff || quittance2 {
            // 指した駒を戻したところで、探索を打ち切る。
            break;
        }
    }

    // 返却。
    (best_movement_hash, alpha)
}
