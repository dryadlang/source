; Dryad AOT LLVM IR (generated)
declare i8* @dryad_variant_add(i8*, i8*)
declare i8* @dryad_variant_get_element(i8*, i8*)
declare i8* @dryad_intrinsic_slow(i16, i8)
declare i8* @dryad_new_array()
declare i8* @dryad_array_push(i8*, i8*)
declare i8* @dryad_array_set(i8*, i8*, i8*)
declare void @dryad_print(i8*)
declare i8* @dryad_box_i32(i32)
declare i8* @dryad_box_f64(double)
declare i8* @dryad_box_bool(i1)
declare i32 @dryad_unbox_i32(i8*)
declare i1 @dryad_variant_eq(i8*, i8*)
declare i1 @dryad_variant_neq(i8*, i8*)
declare double @dryad_unbox_f64(i8*)
declare i1 @dryad_unbox_bool(i8*)
declare i8* @dryad_box_cstring(i8*)
declare i8* @dryad_box_lit_cstring(i8*)

@dryad_lit___dryad_main_0 = private unnamed_addr constant [14 x i8] c"NativeBackend\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [4 x i8] c"VFS\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [26 x i8] c"dryad_file_io_example.txt\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [16 x i8] c"Olá, ficheiro!\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [14 x i8] c"VFS_writeFile\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [13 x i8] c"VFS_readFile\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [15 x i8] c"VFS_removeFile\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [11 x i8] c"VFS_exists\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [12 x i8] c"VFS_dispose\00"

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  %slot2 = alloca i8*, align 8
  %slot3 = alloca i1, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v1 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v1, i8** %slot0, align 8
  %v2 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([26 x i8], [26 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  store i8* %v2, i8** %slot1, align 8
  %v3 = load i8*, i8** %slot0, align 8
  %v4 = load i8*, i8** %slot1, align 8
  %v5 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @dryad_lit___dryad_main_3, i64 0, i64 0))
  %v6 = call i8* @dryad_intrinsic_slow(i16 65417, i8 3)
  %v7 = load i8*, i8** %slot0, align 8
  %v8 = load i8*, i8** %slot1, align 8
  %v9 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  store i8* %v9, i8** %slot2, align 8
  %v10 = load i8*, i8** %slot2, align 8
  %v11 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @dryad_lit___dryad_main_3, i64 0, i64 0))
  %as_i8_0 = bitcast ptr %v10 to i8*
  %as_i8_1 = bitcast ptr %v11 to i8*
  %v12 = call i1 @dryad_variant_eq(i8* %as_i8_0, i8* %as_i8_1)
  store i1 %v12, i1* %slot3, align 8
  %v13 = load i8*, i8** %slot0, align 8
  %v14 = load i8*, i8** %slot1, align 8
  %v15 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  %v16 = load i1, i1* %slot3, align 8
  br i1 %v16, label %bb1, label %bb2

bb1:
  %v17 = load i8*, i8** %slot0, align 8
  %v18 = load i8*, i8** %slot1, align 8
  %v19 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  %as_i8_2 = bitcast ptr %v19 to i8*
  %v20 = call i1 @dryad_unbox_bool(i8* %as_i8_2)
  %v21 = icmp eq i1 %v20, 0
  br label %bb2

bb2:
  %v22 = phi i1 [%v16, %bb0], [%v21, %bb1]
  store i1 %v22, i1* %slot3, align 8
  %v23 = load i8*, i8** %slot0, align 8
  %v24 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  ret i8* null

}

declare i8* @dryad_argv_box_i32(i8**, i32)
declare i8* @dryad_argv_box_f64(i8**, i32)
declare i8* @dryad_argv_box_bool(i8**, i32)
declare i8* @dryad_argv_box_cstring(i8**, i32)
define i32 @main(i32 %argc, i8** %argv) {
entry:
  call i8* @__dryad_main()
  ret i32 0
}
