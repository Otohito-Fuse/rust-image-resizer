use image::DynamicImage;
use image::GenericImage;
use image::GenericImageView;
use image::Rgba;
use std::cmp::min;
use std::path::PathBuf;
use std::time::Instant;

fn main() {
    // 何分の一にするか
    let mut ratio = String::new();
    std::io::stdin().read_line(&mut ratio).ok();
    // u32型に変換
    let ratio: u32 = ratio.trim().parse().ok().unwrap();

    // 入力ファイル名
    let mut input_file_name = String::new();
    std::io::stdin().read_line(&mut input_file_name).ok();

    // 入力するファイルのパスを作成
    let input = PathBuf::from(&("./input/".to_string() + &input_file_name.trim()));

    // 時間計測のための変数
    let mut now = Instant::now();

    // ファイルの入力
    let img_input: DynamicImage = image::open(input).unwrap();

    // ファイル入力にかかった時間をログ
    println!(
        "Reading the file took {} seconds.",
        now.elapsed().as_secs_f64()
    );

    // 出力するファイルのファイル名を作成
    // 一旦ピリオドで区切る
    let mut output_file_name_vec: Vec<&str> = input_file_name.split('.').collect();
    // 拡張子の直前に after_resize と入れる
    output_file_name_vec.insert(output_file_name_vec.len() - 1, "after_resize");
    // ファイル名
    let mut output_file_name = String::new();
    for s in output_file_name_vec {
        output_file_name = output_file_name + "." + s;
    }
    // 余分な先頭のピリオドを取り除く
    output_file_name.remove(0);
    // 出力先のファイルのパスを作成
    let output = PathBuf::from(&("./output/".to_string() + &output_file_name.trim()));

    // リサイズにかかった時間も計測したいのでnowを更新
    now = Instant::now();

    // リサイズする
    let img_output = image_resize(img_input, ratio);

    // リサイズにかかった時間をログ
    println!("Resizing took {} seconds.", now.elapsed().as_secs_f64());

    // ファイルを出力
    img_output.save(output).unwrap();
}

// リサイズ関数
// ratio分の1のサイズにする
fn image_resize(img_input: DynamicImage, ratio: u32) -> DynamicImage {
    // 元画像のサイズ
    let (width, height) = img_input.dimensions();

    // 新しい画像のサイズを決める
    // 割り切れない分は切り上げ
    let new_width: u32 = (width + ratio - 1) / ratio;
    let new_height: u32 = (height + ratio - 1) / ratio;

    // とりあえず画像を生成
    let mut img_output = DynamicImage::new_rgb8(new_width, new_height);

    // 各ピクセルを決定していく
    for i in 0..new_width {
        for j in 0..new_height {
            // 元画像のうち、x座標がi*ratioから(i+1)*ratio-1、y座標がj*ratioから(j+1)*ratio-1までの範囲のピクセルの平均を取る
            // 一旦u32型で合計を取って、割って、そのあとu8型にまた戻す
            let mut red: u32 = 0;
            let mut green: u32 = 0;
            let mut blue: u32 = 0;
            let mut alpha: u32 = 0;

            // widthやheightがratioで割り切れなかった場合は端だけ数が変わるのでそこを加味して上端と下端を設定
            let il = i * ratio;
            let ir = min((i + 1) * ratio, width);
            let jl = j * ratio;
            let jr = min((j + 1) * ratio, height);

            // 一旦足す
            for x in il..ir {
                for y in jl..jr {
                    // get_pixelで一つのピクセルを取ってくる
                    let pixel: Rgba<u8> = img_input.get_pixel(x, y);
                    red += pixel[0] as u32;
                    green += pixel[1] as u32;
                    blue += pixel[2] as u32;
                    alpha += pixel[3] as u32;
                }
            }

            // 割って平均を取る
            red /= (ir - il) * (jr - jl);
            green /= (ir - il) * (jr - jl);
            blue /= (ir - il) * (jr - jl);
            alpha /= (ir - il) * (jr - jl);

            // 新しい画像に平均値を代入
            img_output.put_pixel(
                i,
                j,
                Rgba([red as u8, green as u8, blue as u8, alpha as u8]),
            );
        }
    }
    img_output
}
