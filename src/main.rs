use chrono::prelude::*;
use notify_rust::Notification;
use rand::Rng;
use std::{thread, time::Duration};

fn main() {
    let sentences = [
        "亲亲，乖乖，记得不要一直坐在电脑前哦！起来动动，让小腿肚子舒服一下呀！",
        "宝宝，你坐了这么久，脊椎一定很酸痛吧？和我一起起来走一圈吧，我给你揉揉肩膀哦~",
        "小可爱，不要再沉迷于工作中啦，休息一下，跟我一起伸个懒腰吧~",
        "喵喵，你看你看，这么可爱的小猫咪都知道要经常运动呢！一起跟着小猫咪动一动身体吧！",
        "亲爱的，听说长时间久坐会影响新陈代谢哦！快起来和我一起活动一下吧，让身体更健康哦！",
        "亲亲，乖乖，你不能一直坐着哦！起来活动活动，让身体更健康呢！",
        "宝贝，你坐了这么久，脖子一定很累吧？来和我一起做做颈椎操，让宝宝轻松一下吧~",
        "小可爱，不要老是埋头工作哦，和我一起起来走一圈，透透气，休息一下吧！",
        "喵喵，你看你看，这么可爱的小猫咪都知道要多动动身体呢！和我一起来做几个简单的伸展运动吧！",
        "亲爱的，久坐不仅容易疲劳，还会影响健康哦！快起来和我一起跳几分钟的舞蹈吧，让身体更健康活力哦！",
        "宝贝，你一定很忙，但是要记得定时起来活动哦！来，和我一起做几个简单的运动，让身体更健康有活力呀~",
        "嘘，小宝贝，不要总是坐着不动哦，这样不好！来和我一起起身走几步，透透气，调整一下心情吧！",
        "哎呀，我的小可爱，脚伸得这么长，肯定很酸痛吧！快起身伸展一下吧，我帮你按摩按摩，让你的身体更舒服哦！",
        "咕噜噜，你肚子叫了，是不是太久没活动了呀！来和我一起做几个简单的运动，让你的身体更健康有活力哦！",
        "宝宝，别老是对着电脑啦，眼睛会很疲劳的！来和我一起做几个眼保健操，让你的眼睛更舒服哦！",
        "小宝贝，我看你坐了这么久，来和我一起站起来，扭一扭腰，让身体更舒服哦~",
        "亲亲，不要老是坐着不动哦，和我一起做几个简单的运动，让你的身体更健康有活力呀！我们可以一起跳舞哦~",
        "宝贝，你一定很累了吧！来和我一起做几个轻松的伸展运动，让身体放松一下，心情也会变好哦~",
        "小可爱，看你这么紧张，要注意身体哦！我们来一起做一些简单的深呼吸，让身体更健康有活力呀~",
        "喵喵，你看你看，我都做了几个小猫伸展运动了呢！一起来跟我学学，让身体更灵活哦！"
    ];

    let reminder_interval = Duration::from_secs(1);

    loop {
        let now = Utc::now();
        let (minute, second) = (now.minute(), now.second());

        if minute == 40 && second == 0 {
            let mut rng = rand::thread_rng();
            let random_number: usize = rng.gen_range(0..sentences.len());

            Notification::new()
                .appname("Stand Up!")
                .summary("三點幾了，飲茶先啦！")
                .body(sentences[random_number])
                .sound_name("Glass")
                .show()
                .unwrap();

            thread::sleep(reminder_interval);
        }
    }
}
