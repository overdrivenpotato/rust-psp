use core::ptr::null_mut;
use psp::test_runner::TestRunner;
use psp::vram_alloc::get_vram_allocator;

pub fn test_main(test_runner: &mut TestRunner) {
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

    unsafe {
        let zero_ptr = null_mut();

        let chunk1 = alloc.alloc_sized::<[u8; 4]>(1);
        let chunk2 = alloc.alloc_sized::<[u8; 4]>(1);

        test_runner.check_list(&[
            (
                "first_chunk_addr_zero",
                chunk1.as_mut_ptr_direct_to_vram(),
                psp::sys::sceGeEdramGetAddr(),
            ),
            (
                "second_chunk_addr_zero",
                chunk2.as_mut_ptr_direct_to_vram(),
                psp::sys::sceGeEdramGetAddr().offset(4),
            ),
            (
                "first_chunk_addr_direct",
                chunk1.as_mut_ptr_from_zero(),
                zero_ptr,
            ),
            (
                "second_chunk_addr_direct",
                chunk2.as_mut_ptr_from_zero(),
                zero_ptr.offset(4),
            ),
        ]);

        let muh_item = alloc.move_to_vram([69u8; 16]);

        test_runner.check(
            "vram_moved_addr",
            muh_item.as_mut_ptr(),
            0x4000008 as *const u8 as _,
        );

        test_runner.check("vram_storage_len", muh_item.len(), 16);
        test_runner.check("vram_storage_integrity1", muh_item[4], 69);
        muh_item[15] = 42;
        test_runner.check("vram_storage_integrity2", muh_item[15], 42);
    }
}
