use core::arch::asm;
const SPACE_SIZE: usize = 0x200;
struct stubRec<'a> {
    pos_number: usize,
    pos_name: &'a str,
    stub_description: &'a str,
}
static STUBTABLE: [stubRec;34] = [
	stubRec{pos_number:999,pos_name:"End_Start",stub_description:"End_Start\n"},
	stubRec{pos_number:1,pos_name:"clear_bss_0",stub_description:"os/src/main.rs->clear_bss\n"},
	stubRec{pos_number:2,pos_name:"rust_main_2.println!",stub_description:"os/src/main.rs->rust_main.println\n"},
	stubRec{pos_number:3,pos_name:"rust_main_3.trap::init",stub_description:"os/src/main.rs->rust_main.trap::init\n"},
	stubRec{pos_number:4,pos_name:"rust_main_4.batch::init",stub_description:"os/src/main.rs->rust_main.batch::init\n"},
	stubRec{pos_number:5,pos_name:"rust_main_5.batch::run_next_app",stub_description:"os/src/main.rs->rust_main.batch::run_next_app\n"},
	stubRec{pos_number:6,pos_name:"console_macro_rules_print",stub_description:"os/src/console.rs->console::macro_rules_print!\n"},
	stubRec{pos_number:7,pos_name:"console_macro_rules_println",stub_description:"os/src/console.rs->console::macro_rules_println!\n"},
	stubRec{pos_number:8,pos_name:"trap_init",stub_description:"os/src/trap/mod.rs->trap::init\n"},
	stubRec{pos_number:9,pos_name:"trap_handler_Exception::UserEnvCall",stub_description:"os/src/trap/mod.rs->trap::handler1->syscall\n"},
	stubRec{pos_number:10,pos_name:"trap_handler_Exception::StoreFault",stub_description:"os/src/trap/mod.rs->trap::handler2->StoreFault.println\n"},
	stubRec{pos_number:11,pos_name:"trap_handler_Exception::StoreFault.run_next_app",stub_description:"os/src/trap/mod.rs->trap::handler2->StoreFault.run_next_app\n"},
	stubRec{pos_number:12,pos_name:"trap_handler_Exception::IllegalInstruction.println!",stub_description:"os/src/trap/mod.rs->trap::handler3->IllegalInstruction.println!\n"},
	stubRec{pos_number:13,pos_name:"trap_handler_Exception::IllegalInstruction.run_next_app",stub_description:"os/src/trap/mod.rs->trap::handler3->IllegalInstruction.run_next_app\n"},
	stubRec{pos_number:14,pos_name:"batch::init",stub_description:"os/src/batch.rs->batch::init\n"},
	stubRec{pos_number:15,pos_name:"batch::print_app_info",stub_description:"os/src/batch.rs->batch::print_app_info\n"},
	stubRec{pos_number:16,pos_name:"batch::AppManager::print_app_info",stub_description:"os/src/batch.rs->batch::AppManager::print_app_info\n"},
	stubRec{pos_number:17,pos_name:"batch::lazy_static",stub_description:"os/src/batch.rs->batch::lazy_static\n"},
	stubRec{pos_number:18,pos_name:"batch::run_next_app",stub_description:"os/src/batch.rs->batch::run_next_app\n"},
	stubRec{pos_number:19,pos_name:"batch::KernelStack::push_context",stub_description:"os/src/batch.rs->batch::KernelStack::push_context\n"},
	stubRec{pos_number:20,pos_name:"trap/context.rs::app_init_context",stub_description:"trap/context.rs::app_init_context\n"},
	stubRec{pos_number:21,pos_name:"syscall_SYSCALL_WRITE",stub_description:"os/src/syscall/mod.rs->mod::syscall_SYSCALL_WRITE\n"},
	stubRec{pos_number:22,pos_name:"syscall_SYSCALL_EXIT",stub_description:"os/src/syscall/mod.rs->mod::syscall_SYSCALL_EXIT\n"},
	stubRec{pos_number:23,pos_name:"sys_write",stub_description:"os/src/syscall/fs.rs->fs::sys_write\n"},
	stubRec{pos_number:24,pos_name:"sys_exit",stub_description:"os/src/syscall/process.rs->process::sys_exit\n"},
	stubRec{pos_number:25,pos_name:"UPSafeCell_new",stub_description:"os/src/sync/up.rs->up::new\n"},
	stubRec{pos_number:26,pos_name:"UPSafeCell_exclusive_access",stub_description:"os/src/sync/up.rs->up::exclusive_access\n"},
	stubRec{pos_number:27,pos_name:"sbi_console_putchar",stub_description:"os/src/sbi.rs->up::console_putchar\n"},
	stubRec{pos_number:28,pos_name:"sbi_console_getchar",stub_description:"os/src/sbi.rs->sbi::console_getchar\n"},
	stubRec{pos_number:29,pos_name:"sbi_shutdown",stub_description:"os/src/sbi.rs->sbi::shutdown\n"},
	stubRec{pos_number:30,pos_name:"console_write_str",stub_description:"os/src/console.rs->console::write_str\n"},
	stubRec{pos_number:31,pos_name:"console_print",stub_description:"os/src/console.rs->console::print\n"},
	stubRec{pos_number:32,pos_name:"panic",stub_description:"os/src/lang_items.rs->lang_items::panic\n"},
	stubRec{pos_number:33,pos_name:"batch::run_next_app.__restore",stub_description:"os/src/batch.rs->batch::run_next_app.__restore\n"},
];

struct RecLogSpace {
    next: usize,
    space: [usize; SPACE_SIZE],
}

impl RecLogSpace {
   unsafe fn write_log(&mut self, log_id: usize) {
        self.space[self.next]=log_id;
        self.next= self.next+1;
	asm!("fence.i");
     }
   pub fn print_log(&self) {
	let mut i=0;
	print!("[");
	loop {
		if self.space[i] == 5555 {
            		break;
        	}
		else {
			print!("{}",self.space[i]);
			i += 1;
			if self.space[i] != 5555 {
				print!(", ");
			}
		}
	}
	print!("]\n");
	i=0;
 	loop {
        	if self.space[i] == 5555 {
            		break;
        	}
		print!("------{} {}",STUBTABLE[self.space[i]].pos_number,STUBTABLE[self.space[i]].stub_description);
        	i += 1;
		if i==20000 {break;}
    	}
     }
 }

static mut reclogspace:RecLogSpace=RecLogSpace{
          		next: 0,
          		space: [5555; SPACE_SIZE],
};
pub fn write_event(log_id: usize){
    unsafe {
        reclogspace.write_log(log_id);
    }
}

pub fn print_log(){
    unsafe {reclogspace.print_log();
    }
}
