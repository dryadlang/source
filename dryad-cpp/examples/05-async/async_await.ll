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

@dryad_lit_add_0 = private unnamed_addr constant [16 x i8] c"Promise_resolve\00"
@dryad_lit___lambda_0_0 = private unnamed_addr constant [6 x i8] c"order\00"
@dryad_lit___dryad_main_0 = private unnamed_addr constant [14 x i8] c"Promise_flush\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [10 x i8] c"async add\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [16 x i8] c"Promise_resolve\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [5 x i8] c"then\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [15 x i8] c"EventLoop_pump\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [16 x i8] c"microtask order\00"

define i8* @add(i8* %arg0, i8* %arg1) {
  %slot0 = alloca i32, align 8
  %slot1 = alloca i32, align 8
  %slot2 = alloca i8*, align 8
  %slot3 = alloca i8*, align 8
  %p0 = call i32 @dryad_unbox_i32(i8* %arg0)
  store i32 %p0, i32* %slot0, align 4
  %p1 = call i32 @dryad_unbox_i32(i8* %arg1)
  store i32 %p1, i32* %slot1, align 4
  br label %bb0

bb0:
  %v0 = load i32, i32* %slot0, align 8
  %v1 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v2 = call i8* @dryad_intrinsic_slow(i16 88, i8 1)
  store i8* %v2, i8** %slot2, align 8
  %v3 = load i32, i32* %slot1, align 8
  %v4 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v5 = call i8* @dryad_intrinsic_slow(i16 88, i8 1)
  store i8* %v5, i8** %slot3, align 8
  %v6 = load i8*, i8** %slot2, align 8
  %v7 = load i8*, i8** %slot3, align 8
  %ga_ul8 = call i32 @dryad_unbox_i32(i8* %v6)
  %ga_ur8 = call i32 @dryad_unbox_i32(i8* %v7)
  %ga_l8 = call i8* @dryad_box_i32(i32 %ga_ul8)
  %ga_r8 = call i8* @dryad_box_i32(i32 %ga_ur8)
  %v8 = call i8* @dryad_variant_add(i8* %ga_l8, i8* %ga_r8)
  ret i8* %v8

}

define i8* @__lambda_0(i8* %arg0) {
  %slot0 = alloca i8*, align 8
  store i8* %arg0, i8** %slot0, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v1 = add i32 0, 1
  %v2 = call i8* @dryad_intrinsic_slow(i16 74, i8 2)
  %v3 = load i8*, i8** %slot0, align 8
  ret i8* %v3

}

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = add i32 0, 20
  %v1 = add i32 0, 22
  %v2 = call i8* @dryad_intrinsic_slow(i16 65280, i8 2)
  store i8* %v2, i8** %slot0, align 8
  %v3 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v4 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @dryad_lit___dryad_main_1, i64 0, i64 0))
  call void @dryad_print(i8* %v4)
  %v5 = inttoptr i64 0 to i8*
  %v6 = load i8*, i8** %slot0, align 8
  %v7 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v7)
  %v8 = inttoptr i64 0 to i8*
  %v9 = call i8* @dryad_new_array()
  store i8* %v9, i8** %slot1, align 8
  %v10 = add i32 0, 1
  %v11 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v12 = inttoptr i64 0 to i8*
  %v13 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v14 = load i8*, i8** %slot1, align 8
  %v15 = add i32 0, 0
  %v16 = call i8* @dryad_intrinsic_slow(i16 74, i8 2)
  %v17 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v18 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  call void @dryad_print(i8* %v18)
  %v19 = inttoptr i64 0 to i8*
  %v20 = load i8*, i8** %slot1, align 8
  %v21 = add i32 0, 0
  %ge_idx22 = call i8* @dryad_box_i32(i32 %v21)
  %v22 = call i8* @dryad_variant_get_element(i8* %v20, i8* %ge_idx22)
  %v23 = call i32 @dryad_unbox_i32(i8* %v22)
  %print_box24 = call i8* @dryad_box_i32(i32 %v23)
  call void @dryad_print(i8* %print_box24)
  %v24 = inttoptr i64 0 to i8*
  %v25 = load i8*, i8** %slot1, align 8
  %v26 = add i32 0, 1
  %ge_idx27 = call i8* @dryad_box_i32(i32 %v26)
  %v27 = call i8* @dryad_variant_get_element(i8* %v25, i8* %ge_idx27)
  %v28 = call i32 @dryad_unbox_i32(i8* %v27)
  %print_box29 = call i8* @dryad_box_i32(i32 %v28)
  call void @dryad_print(i8* %print_box29)
  %v29 = inttoptr i64 0 to i8*
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
