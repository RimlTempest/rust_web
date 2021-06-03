// OSバージョンを返却
pub mod index;
// userIDを取得(Pathから値を取得)
pub mod get_user_id;
// groupID、userIDを取得(Pathから複数の値を取得)
pub mod get_group_id;
// HoloQueryモデルをもとにQueryを取得
pub mod get_holo_member;
// HoloBodyをモデルをもとにPost
pub mod post_holo_member;
// レスポンスをモデルをもとに取得
pub mod post_project;