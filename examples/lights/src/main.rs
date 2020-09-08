#![no_std]
#![no_main]

use core::f32::consts::PI;

use psp::{
    Align16,
    sys::{
        self, ScePspFVector3, TexturePixelFormat, DisplayPixelFormat, GuContextType,
        GuState, DepthFunc, GuSyncMode, GuSyncBehavior, ClearBuffer, GuPrimitive,
        VertexType, LightType::Pointlight, LightComponent, MatrixMode, ShadingModel,
        FrontFaceDirection,
    },
    math::{
        cosf32, sinf32,
    },
    vram_alloc::get_vram_allocator,
    BUF_WIDTH, SCREEN_WIDTH, SCREEN_HEIGHT
};


psp::module!("lights", 1, 1);

static mut LIST: Align16<[u32; 0x40000]> = Align16([0; 0x40000]);

const GRID_COLUMNS: usize = 32;
const GRID_ROWS: usize = 32;
const GRID_SIZE: f32 = 10.0;

static mut GRID_VERTICES: Align16<[NPVertex; GRID_COLUMNS * GRID_ROWS]> = Align16(
    [
    NPVertex {
        nx: 0.0, ny: 0.0, nz: 0.0,
        px: 0.0, py: 0.0, pz: 0.0,
    }; GRID_COLUMNS * GRID_ROWS
    ]);

static mut GRID_INDICES: Align16<[u16; (GRID_COLUMNS-1)*(GRID_ROWS-1) * 6]> = Align16(
    [0u16; (GRID_COLUMNS-1)*(GRID_ROWS-1) * 6]
);

const TORUS_SLICES: usize = 48;
const TORUS_ROWS: usize = 48;
const TORUS_RADIUS: f32 = 1.0;
const TORUS_THICKNESS: f32 = 0.5;
const LIGHT_DISTANCE: f32 = 3.0;

#[derive(Copy, Clone)]
#[repr(C, align(4))]
struct NPVertex {
    nx: f32, ny: f32, nz: f32,
    px: f32, py: f32, pz: f32,
}

static mut TORUS_VERTICES: Align16<[NPVertex; TORUS_SLICES * TORUS_ROWS]> = Align16(
    [
    NPVertex {
        nx: 0.0, ny: 0.0, nz: 0.0,
        px: 0.0, py: 0.0, pz: 0.0,
    }; TORUS_SLICES * TORUS_ROWS
    ]);

static mut TORUS_INDICES: Align16<[u16; TORUS_SLICES * TORUS_ROWS * 6]> = Align16(
    [0u16; TORUS_SLICES * TORUS_ROWS * 6]
);

static COLORS: [u32;4] = [
    0xffff0000,
    0xff00ff00,
    0xff0000ff,
    0xffff00ff,
];

fn generate_grid(columns: u32, rows: u32, width: f32, depth: f32, mut vertices: *mut u8, mut indices: *mut u16, size: usize, texture: Option<usize>, color: Option<usize>, normal: Option<usize>, position: Option<usize>) {
    let ic = 1.0 / columns as f32;
    let ir = 1.0 / rows as f32;

    unsafe {
        for j in 0..rows {
            for i in 0..columns {
                let mut offset: usize = 0;

                if texture != None { 
                    let texcoords: *mut f32 = vertices.add(offset) as *mut f32;
                    *texcoords = ic * (i as f32);
                    *texcoords.add(1) = ir * (j as f32);
                    offset += texture.unwrap();
                }

                if color != None {
                    let col: *mut u32 = vertices.add(offset) as *mut u32;
                    *col = 0xffff_ffff;
                    offset += color.unwrap();
                }

                if normal != None {
                    let normals: *mut f32 = vertices.add(offset) as *mut f32;
                    *normals = 0.0;
                    *normals.add(1) = 1.0;
                    *normals.add(2) = 0.0;
                    offset += normal.unwrap();
                }

                if position != None {
                    let pos: *mut f32 = vertices.add(offset) as *mut f32;
                    *pos = ((ic * i as f32) - 0.5) * width;
                    *pos.add(1) = 0.0;
                    *pos.add(2) = ((ir * j as f32)-0.5) * depth; 
                }

                vertices = vertices.add(size);
            }
        }
        for j in 0..rows-1 {
            for i in 0..columns-1 {
                *indices = (i + j * columns) as u16;            
                indices = indices.add(1);
                *indices = ((i+1) + j * columns) as u16;
                indices = indices.add(1);
                *indices = (i + (j+1) * columns) as u16;
                indices = indices.add(1);

                *indices = ((i+1) + j * columns) as u16;
                indices = indices.add(1);
                *indices = ((i+1) + (j+1) * columns) as u16;
                indices = indices.add(1);
                *indices = (i + (j+1) * columns) as u16;
                indices = indices.add(1);
            }
        }
    }
}

fn generate_grid_np(columns: u32, rows: u32, width: f32, depth: f32, vertices: *mut NPVertex, indices: *mut u16) {
   generate_grid(columns, rows, width, depth, vertices as *mut u8, indices, core::mem::size_of::<NPVertex>(), None, None, Some(3 * core::mem::size_of::<f32>()), Some(3*core::mem::size_of::<f32>())); 
}

fn generate_torus(slices: u32, rows: u32, radius: f32, thickness: f32, mut vertices: *mut u8, mut indices: *mut u16, size: usize, texture: Option<usize>, color: Option<usize>, normal: Option<usize>, position: Option<usize>) {
    unsafe {
        for j in 0..slices {
            for i in 0..rows {
                let s = i as f32 + 0.5;
                let t = j as f32;
                let cs: f32;
                let ct: f32;
                let ss: f32;
                let st: f32;
                let mut offset: usize = 0;

                cs = cosf32(s * (2.0*PI) / slices as f32);
                ct = cosf32(t * (2.0*PI) / rows as f32);
                ss = sinf32(s * (2.0*PI) / slices as f32);
                st = sinf32(t * (2.0*PI) / rows as f32);

                if texture != None {
                    let texcoords: *mut f32 = (vertices).add(offset) as *mut f32;
                    *texcoords = cs * ct; 
                    *texcoords.add(1) = cs * st;
                    offset += texture.unwrap();
                }

                if color != None {
                    let col: *mut u32 = (vertices).add(offset) as *mut u32;
                    let r: u32 = 128 + (cs * ct) as u32 * 127;
                    let g: u32 = 128 + (cs * st) as u32 * 127;
                    let b: u32 = 128 + ss as u32 * 127;
                    *col = (0xff << 24) | (b << 16) | (g << 8) | r;
                    offset += color.unwrap();
                }

                if normal != None {
                    let normals: *mut f32 = (vertices).add(offset) as *mut f32;
                    *normals = cs * ct;
                    *normals.add(1) = cs * st;
                    *normals.add(2) = ss;
                    offset += normal.unwrap();
                }

                if position != None {
                    let pos: *mut f32 = (vertices).add(offset) as *mut f32;
                    *pos = (radius + thickness * cs) * ct;
                    *pos.add(1) = (radius + thickness * cs) * st;
                    *pos.add(2) = thickness * ss;
                }

                vertices = vertices.add(size);
            }
        }

        for j in 0..slices {
            for i in 0..rows {
                let i1: u32 = (i+1) % rows;
                let j1: u32 = (j+1) % slices;

                *indices = (i + j * rows) as u16;
                indices = indices.add(1);
                *indices = (i1 + j * rows) as u16;
                indices = indices.add(1);
                *indices = (i + j1 * rows) as u16;
                indices = indices.add(1);

                *indices = (i1 + j * rows) as u16;
                indices = indices.add(1);
                *indices = (i1 + j1 * rows) as u16;
                indices = indices.add(1);
                *indices = (i + j1 * rows) as u16;
                indices = indices.add(1);
            }
        }
    }
}

fn generate_torus_np(slices: u32, rows: u32, radius: f32, thickness: f32, vertices: *mut NPVertex, indices: *mut u16) {
    generate_torus(slices, rows, radius, thickness, vertices as *mut u8, indices, core::mem::size_of::<NPVertex>(), None, None, Some(3 * core::mem::size_of::<f32>()), Some(3 * core::mem::size_of::<f32>()));   
}

fn psp_main() {
    psp::enable_home_button();

    let np_vertex_format: VertexType = VertexType::NORMAL_32BITF | VertexType::VERTEX_32BITF;

    unsafe {
        sys::sceKernelChangeCurrentThreadAttr(0, sys::ThreadAttributes::VFPU);
        generate_grid_np(GRID_COLUMNS as u32, GRID_ROWS as u32, GRID_SIZE, GRID_SIZE, &mut GRID_VERTICES.0 as *mut _ as _, &mut GRID_INDICES.0 as *mut _ as _);
        generate_torus_np(TORUS_SLICES as u32, TORUS_ROWS as u32, TORUS_RADIUS, TORUS_THICKNESS, &mut TORUS_VERTICES.0 as *mut _ as _, &mut TORUS_INDICES.0 as *mut _ as _);
        sys::sceKernelDcacheWritebackAll();

        let mut allocator = get_vram_allocator().unwrap();
        let fbp0 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
        let fbp1 = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm8888).as_mut_ptr_from_zero();
        let zbp = allocator.alloc_texture_pixels(BUF_WIDTH, SCREEN_HEIGHT, TexturePixelFormat::Psm4444).as_mut_ptr_from_zero();

        sys::sceGuInit();        
        sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut _ as _);
        sys::sceGuDrawBuffer(DisplayPixelFormat::Psm8888, fbp0 as _, BUF_WIDTH as i32);
        sys::sceGuDispBuffer(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32, fbp1 as _, BUF_WIDTH as i32);
        sys::sceGuDepthBuffer(zbp as _, BUF_WIDTH as i32);
        sys::sceGuOffset(2048 - (SCREEN_WIDTH / 2), 2048 - (SCREEN_HEIGHT / 2));
        sys::sceGuViewport(2048, 2048, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuDepthRange(0xc350, 0x2710);
        sys::sceGuScissor(0, 0, SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);
        sys::sceGuEnable(GuState::ScissorTest);
        sys::sceGuDepthFunc(DepthFunc::GreaterOrEqual);
        sys::sceGuFrontFace(FrontFaceDirection::Clockwise);
        sys::sceGuShadeModel(ShadingModel::Smooth);
        sys::sceGuEnable(GuState::DepthTest);
        sys::sceGuEnable(GuState::CullFace);
        sys::sceGuEnable(GuState::ClipPlanes);
        sys::sceGuEnable(GuState::Lighting);
        sys::sceGuEnable(GuState::Light0);
        sys::sceGuEnable(GuState::Light1);
        sys::sceGuEnable(GuState::Light2);
        sys::sceGuEnable(GuState::Light3);
        sys::sceGuFinish();
        sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
        sys::sceDisplayWaitVblankStart();
        sys::sceGuDisplay(true);

        let mut val = 0.0;

        loop {
            sys::sceGuStart(GuContextType::Direct, &mut LIST.0 as *mut _ as _);
            sys::sceGuClearColor(0x554433);
            sys::sceGuClearDepth(0);
            sys::sceGuClear(ClearBuffer::COLOR_BUFFER_BIT | ClearBuffer::DEPTH_BUFFER_BIT);

            for i in 0..4 {
                let pos = ScePspFVector3 {
                    x: cosf32(i as f32 * (PI / 2.0) + val as f32 * (PI / 180.0)) * LIGHT_DISTANCE,
                    y: 0.0,
                    z: sinf32(i as f32 * (PI / 2.0) + val as f32 * (PI / 180.0)) * LIGHT_DISTANCE                
                }; 
                sys::sceGuLight(i, Pointlight, LightComponent::DIFFUSE | LightComponent::SPECULAR, &pos);
                sys::sceGuLightColor(i, LightComponent::DIFFUSE, COLORS[i as usize]);
                sys::sceGuLightColor(i, LightComponent::SPECULAR, 0xffff_ffff);
                sys::sceGuLightAtt(i, 0.0, 1.0, 0.0);
            }

            sys::sceGuSpecular(12.0);
            sys::sceGuAmbient(0x0022_2222);

            sys::sceGumMatrixMode(MatrixMode::Projection);
            sys::sceGumLoadIdentity();
            sys::sceGumPerspective(75.0, 16.0/9.0, 1.0, 1000.0);

            sys::sceGumMatrixMode(MatrixMode::View);
            let pos = ScePspFVector3 {
                x: 0.0,
                y: 0.0,
                z: -3.5
            };
            sys::sceGumLoadIdentity();
            sys::sceGumTranslate(&pos);
            
            sys::sceGumMatrixMode(MatrixMode::Model);
            let pos = ScePspFVector3 {
                x: 0.0,
                y: -1.5,
                z: 0.0
            };
            sys::sceGumLoadIdentity();
            sys::sceGumTranslate(&pos);

            sys::sceGuColor(0xff7777);
            sys::sceGumDrawArray(GuPrimitive::Triangles,
                np_vertex_format | VertexType::INDEX_16BIT | VertexType::TRANSFORM_3D, (core::mem::size_of_val(&GRID_INDICES.0) / core::mem::size_of::<u16>()) as i32, &GRID_INDICES.0 as *const _ as _, &GRID_VERTICES.0 as *const _ as _);

            sys::sceGumMatrixMode(MatrixMode::Model);
            let rot = ScePspFVector3 {
                x: val * 0.79 * (PI / 180.0), 
                y: val * 0.98 * (PI / 180.0),
                z: val * 1.32 * (PI / 180.0),
            };
            sys::sceGumLoadIdentity();
            sys::sceGumRotateXYZ(&rot);

            sys::sceGuColor(0xffffff);
            sys::sceGumDrawArray(GuPrimitive::Triangles, np_vertex_format | VertexType::INDEX_16BIT | VertexType::TRANSFORM_3D, (core::mem::size_of_val(&TORUS_INDICES.0) / core::mem::size_of::<u16>()) as i32, &TORUS_INDICES.0 as *const _ as _, &TORUS_VERTICES.0 as *const _ as _);
            sys::sceGuFinish();
            sys::sceGuSync(GuSyncMode::Finish, GuSyncBehavior::Wait);
            
            sys::sceDisplayWaitVblankStart();
            sys::sceGuSwapBuffers();

            val += 1.0;
        }
    }
}

