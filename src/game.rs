use std::io;

use crate::common;

use self::board::{Board, turn::CtrlPara};

pub mod unit;
pub mod board;
pub mod skill;
pub mod ai;

const HELP : &str = "更多帮助：
help --game -> 游戏说明
help --skill -> 技能说明
help --screen -> 显示说明
help --bound -> 捆绑说明

操作说明：
save -> 保存
load -> 载入
undo -> 撤销至上一次的选择
back -> 撤销至上一帧
在选择界面，输入对应数字选择相应选择，直接按回车则系统自动行动。
";

const HELP_GAME : &str = "游戏说明：
游戏目标：
击败奎诺和她的三个杂鱼小弟。

胜负条件：
当一方全员被完全捆绑（举白旗）时，另一方获胜。

属性说明：
角色的基本属性为力量、灵巧和敏捷三维。
力量决定攻击力、防御力、压制力、挣扎力、捆绑力、反捆力、挣脱力等。
灵巧决定了命中率，以及捆绑/解绑的效率。
敏捷决定了回避率，以及出手顺序。
角色受到伤害后会增加自身受伤值（可以理解为反向的血条），负伤值每累计到5点，三维-1。
角色的其它所有衍生属性都来自于三维以及自身的异常状态。
朝向：每个角色都有一个朝向，当面对来自后背的袭击时，自身回避与防御减半。

异常状态：
眩晕：三维降至0，下次行动后解除
倒地：倒地后很多技能无法发动，正常情况下会自动起身
束缚：束缚有9个部位，不同部位的束缚组合对各衍生属性影响不同

游戏流程：
每回合，速度最快的先动，执行一个主要技能后，回合结束。
如自身状态无异常，则可以选择等待，等待后会在所有人行动完再动。如多人等待，则速度最快的最后动。
";

const HELP_SKILL : &str = "技能说明：
主要技能：
1. 挥拳
打伤害的技能；
可移动攻击，不能越过敌方站立的角色打击敌后方角色；
有概率击晕对手（击晕后自动倒地）。
2. 擒拿
只有自身命中大于对手回避，且自身压制力大于对手挣扎力时，才能发动；
可移动攻击，不能越过敌方站立的角色打击敌后方角色；
发动后对方倒地，且我方处于擒拿敌方的状态；
擒拿状态如果能保持到下次自己的回合，可对该角色进行捆绑后继续选择下一个主要技能执行；
擒拿的打断方法是：有角色移动至擒拿者与目标之间，或者擒拿者自身倒地，或者目标角色挣扎成功。
3. 解绑
解绑带有束缚的队友；
可先移动，不能越过敌方站立的角色。
4. 脱缚
自身有束缚且手腕不被捆绑时可使用，解绑自身

自动技能：
自身回合开始时，如被擒拿则自动判定能否挣扎成功。
接下来，如有束缚，自动尝试挣脱。
接下来，如果倒地，在脱离擒拿且自身上肢与下肢不都有束缚时，可站起来。
一旦存在上述情况，则本回合不可等待。
";

const HELP_SCREEN : &str = "显示说明
捆绑标记
[@O-@-0O@]从左到右依次代表：
五花大绑、手臂、后颈<-->手腕、手腕、手腕<-->脚腕、大腿、小腿、脚腕 的束缚。
如有后颈<-->脚腕，则-会替换为=显示。
🏳️：战败，被完全束缚
💫：眩晕
🧎：倒下
👆/👇：处于擒拿状态
↑/↓：朝向

在选择主要行动时：
命回：在当前回合人那里，显示的是命中，而在敌人那里，显示的是闪避
攻防：在当前回合人那里，显示的是攻击力，而在敌人那里，显示的是防御力
压挣：在当前回合人那里，显示的是压制力，而在敌人那里，显示的是挣扎力
";

const HELP_BOUND : &str = "捆绑说明
完全捆绑：
要求五花大绑、手臂、后颈<-->手腕、手腕、后颈<-->脚腕、大腿、小腿、脚腕 的束缚。

捆绑前置条件：
手臂：必须先捆绑五花大绑
后颈<-->手腕：必须先捆绑五花大绑+手腕
后颈<-->脚腕：必须先捆绑五花大绑+脚腕
手腕<-->脚腕：必须先捆绑手腕+脚腕

冲突：
后颈<-->手腕 与 手腕<-->脚腕 二者无法共存，为实现完全捆绑，必须先解绑 手腕<-->脚腕。

捆绑建议：
建议优先捆绑 手腕、脚腕、手腕<-->脚腕，最高效地降低对手的行动能力。

";

pub struct Game {
    board : Board,
    history : Vec<(Board, bool)>,
}

impl Game {
    pub fn new_team(seed : u64) -> Self {
        Self {
            board: Board::new_team_theme(seed),
            history : vec![],
        }
    }


    pub fn new_solo_human(seed : u64, str1 : i32, dex1 : i32, agi1 : i32, str2 : i32, dex2 : i32, agi2 : i32) -> Self {
        Self {
            board: Board::new_solo(seed, true, str1, dex1, agi1, str2, dex2, agi2),
            history : vec![],
        }
    }

    pub fn new_solo_auto(seed : u64, str1 : i32, dex1 : i32, agi1 : i32, str2 : i32, dex2 : i32, agi2 : i32) -> Self {
        Self {
            board: Board::new_solo(seed, false, str1, dex1, agi1, str2, dex2, agi2),
            history : vec![],
        }
    }

    pub fn new() -> Self {
        Self {
            board: Board::new_stage_1(114514),
            history : vec![],
        }
    }

    pub fn main_auto(&mut self) -> Option<bool> {
        let mut count = 0;
        while self.board.get_turn() < 100 {
            let mut para = CtrlPara::new();
            para.need_show = false;
            let result = self.board.continue_turn(para);

            // 判断胜负
            let winner = result.winner();
            if winner.is_some() {
                return winner
            }

            self.board.set_to_start();
            count += 1;
            if count > 1000 {
                panic!("Too many loop!")
            }
        }
        None
    }

    pub fn main_loop(&mut self) -> Option<bool> {
        let mut result = self.board.continue_turn(CtrlPara::new());

        loop  {
            // 判断胜负
            let winner = result.winner();
            if winner.is_some() {
                return winner
            }

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let strs: Vec<&str> = input.split_whitespace().collect();

            if strs.len() > 0 {
                let mut load_para = CtrlPara::new();
                load_para.is_load = true;
                match strs[0] {
                    "save" => {
                        self.save();
                        println!("保存成功！");
                        continue;
                    },
                    "load" => {
                        self.load();
                        self.history = vec!();
                        result = self.board.continue_turn(load_para);
                        continue;
                    },
                    "undo" => {
                        while let Some((b, is_choose)) = self.history.pop() {
                            if is_choose {
                                self.board = b;
                                result = self.board.continue_turn(load_para);
                                break;
                            }
                        }
                        continue;
                    }
                    "back" => {
                        if let Some((b, _)) = self.history.pop() 
                        {
                            self.board = b;
                            result = self.board.continue_turn(load_para);
                        }else{
                            println!("初始状态，撤销失败");
                        }
                        continue;
                    }
                    "help" => {
                        match strs.get(1) {
                            Some(&"--game") => println!("{}", HELP_GAME),
                            Some(&"--skill") => println!("{}", HELP_SKILL),
                            Some(&"--screen") => println!("{}", HELP_SCREEN),
                            Some(&"--bound") => println!("{}", HELP_BOUND),
                            _ => println!("{}", HELP),
                        }
                        continue;
                    }
                    _ => (),
                }
            }

            match result.get_choose() {
                Some(chooses) => {
                    if strs.len() == 0 {
                        println!("执行默认选项");
                        let mut para_auto = CtrlPara::new();
                        para_auto.force_auto = true;
                        para_auto.is_load = true;
                        result = self.board.continue_turn(para_auto);
                    }else{
                        let i = strs[0];
                        match i.parse::<usize>() {
                            Err(_) => println!("请输入一个自然数！"),
                            Ok(i) => {
                                let num = chooses.len();
                                if i >= num {
                                    println!("数值越界！")
                                }else {
                                    self.history.push((self.board.clone(), true));
                                    result = self.board.response_choose(CtrlPara::new(), chooses[i].clone());
                                }
                            }, 
                        }
                    }
                },
                None => {
                    self.history.push((self.board.clone(), false));
                    self.board.set_to_start();
                    result = self.board.continue_turn(CtrlPara::new());
                },
            }
        }
    }
}

impl Game {
    fn save(&self) {
        let save_path = "assets/saves/save_0.ddg";
        let serialized = serde_json::to_string(&self.board).unwrap();
        
        if let Err(any) = common::save_file(save_path, serialized) {
            println!("{}", any);
        }
    }

    fn load(&mut self) {
        let save_path = "assets/saves/save_0.ddg";
        
        match common::load_file(save_path) {
            Ok(s) => self.board = serde_json::from_str(&s).expect("Board struct changed! can not load!"),
            Err(any) => println!("{}", any),
        }
    }
}