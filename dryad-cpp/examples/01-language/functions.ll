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

@dryad_lit_factorial_0 = private unnamed_addr constant [10 x i8] c"factorial\00"
@dryad_lit___dryad_main_0 = private unnamed_addr constant [4 x i8] c"add\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [8 x i8] c"closure\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [7 x i8] c"double\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [10 x i8] c"factorial\00"

define i8* @add(i8* %arg0, i8* %arg1) {
  %slot0 = alloca i32, align 8
  %slot1 = alloca i32, align 8
  %p0 = call i32 @dryad_unbox_i32(i8* %arg0)
  store i32 %p0, i32* %slot0, align 4
  %p1 = call i32 @dryad_unbox_i32(i8* %arg1)
  store i32 %p1, i32* %slot1, align 4
  br label %bb0

bb0:
  %v0 = load i32, i32* %slot0, align 8
  %v1 = load i32, i32* %slot1, align 8
  %v2 = add i32 %v0, %v1
  %boxed2 = call i8* @dryad_box_i32(i32 %v2)
  ret i8* %boxed2

}

define i8* @makeMultiplier(i8* %arg0) {
  %slot0 = alloca i32, align 8
  %p0 = call i32 @dryad_unbox_i32(i8* %arg0)
  store i32 %p0, i32* %slot0, align 4
  br label %bb0

bb0:
  %v0 = inttoptr i64 0 to i8*
  ret i8* %v0

}

define i8* @factorial(i8* %arg0) {
  %slot0 = alloca i32, align 8
  %p0 = call i32 @dryad_unbox_i32(i8* %arg0)
  store i32 %p0, i32* %slot0, align 4
  br label %bb0

bb0:
  %v0 = load i32, i32* %slot0, align 8
  %v1 = add i32 0, 1
  %v2 = icmp sle i32 %v0, %v1
  br i1 %v2, label %bb1, label %bb2

bb1:
  %v3 = add i32 0, 1
  %boxed1 = call i8* @dryad_box_i32(i32 %v3)
  ret i8* %boxed1

bb2:
  %v4 = load i32, i32* %slot0, align 8
  %v5 = load i32, i32* %slot0, align 8
  %v6 = add i32 0, 1
  %v7 = call i8* @dryad_intrinsic_slow(i16 65297, i8 2)
  %v8 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v9 = call i8* @dryad_intrinsic_slow(i16 65298, i8 2)
  ret i8* %v9

}

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_0, i64 0, i64 0))
  call void @dryad_print(i8* %v0)
  %v1 = inttoptr i64 0 to i8*
  %v2 = add i32 0, 20
  %v3 = add i32 0, 22
  %v4 = call i8* @dryad_intrinsic_slow(i16 65280, i8 2)
  call void @dryad_print(i8* %v4)
  %v5 = inttoptr i64 0 to i8*
  %v6 = add i32 0, 2
  %v7 = call i8* @dryad_intrinsic_slow(i16 65281, i8 1)
  store i8* %v7, i8** %slot0, align 8
  %v8 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  call void @dryad_print(i8* %v8)
  %v9 = inttoptr i64 0 to i8*
  %v10 = add i32 0, 21
  %v11 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  call void @dryad_print(i8* %v11)
  %v12 = inttoptr i64 0 to i8*
  %v13 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  call void @dryad_print(i8* %v13)
  %v14 = inttoptr i64 0 to i8*
  %v15 = add i32 0, 5
  %v16 = call i8* @dryad_intrinsic_slow(i16 65282, i8 1)
  call void @dryad_print(i8* %v16)
  %v17 = inttoptr i64 0 to i8*
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
