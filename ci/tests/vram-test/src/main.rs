#![no_std]
#![no_main]

use psp::test_runner::TestRunner;
use psp::vram_alloc::get_vram_allocator;

psp::module!("vram_test", 1, 1);

fn psp_main() {
    psp::enable_home_button();

    let mut test_runner = TestRunner::new_file_runner();
    test_runner.start();

    let mut alloc = get_vram_allocator().unwrap();
    test_runner.pass("allocator_initialization", "Received VRAM allocator.");

    let fake_alloc = get_vram_allocator();
    match fake_alloc {
        Ok(_) => test_runner.fail(
            "allocator_doubling_prevention",
            "Received second VRAM allocator! Singleton is not working.",
        ),
        Err(_) => test_runner.pass(
            "allocator_doubling_prevention",
            "VRAM allocator singleton functional.",
        ),
    }

    // TODO: have no safe functions which allow uninitialized mem
    unsafe {
        let chunk1 = alloc.alloc_sized::<[u8; 4]>(1);
        let chunk2 = alloc.alloc_sized::<[u8; 4]>(1);

        test_runner.check_list(&[
            (
                "first_chunk_addr",
                chunk1.as_mut_ptr(),
                psp::sys::sceGeEdramGetAddr(),
            ),
            (
                "second_chunk_addr",
                chunk2.as_mut_ptr(),
                psp::sys::sceGeEdramGetAddr().offset(4),
            ),
        ]);

        let muh_item = alloc.move_to_vram([69u8; 16]);

        test_runner.check("vram_storage_len", muh_item.len(), 16);
        test_runner.check("vram_storage_integrity1", muh_item[4], 69);
        test_runner.check("vram_storage_integrity2", muh_item[15], 69);
    }
    test_runner.finish();
}
