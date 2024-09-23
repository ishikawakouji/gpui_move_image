# gpui_move_image
Move the image according to mouse movements

zedの中の GPUI で、画像を動かすアプリを試してみた。

追加で、ホイールによる拡大縮小もできるようにした。

## 試し方

1. GPUI の examples にある gif_viewer をまねる
2. マウスイベントを当てていく
    1. まずは、mouse_downを当てて、ちゃんと動くかどうか
    2. mouse_up, mouse_move を順に加える
3. mouse_move の移動量を画像のマージンにちゃんと加える
