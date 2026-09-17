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


define i8* @compute_sum(i8* %arg0) {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i32, align 8
  %slot2 = alloca i32, align 8
  store i8* %arg0, i8** %slot0, align 8
  br label %bb0

bb0:
  %v0 = add i32 0, 0
  store i32 %v0, i32* %slot1, align 8
  %v1 = add i32 0, 0
  store i32 %v1, i32* %slot2, align 8
  br label %bb1

bb1:
  %v2 = load i32, i32* %slot2, align 8
  %v3 = load i8*, i8** %slot0, align 8
  %v4 = call i32 @dryad_unbox_i32(i8* %v3)
  %v5 = icmp slt i32 %v2, %v4
  br i1 %v5, label %bb2, label %bb3

bb2:
  %v6 = load i32, i32* %slot1, align 8
  %v7 = load i32, i32* %slot2, align 8
  %v8 = add i32 %v6, %v7
  store i32 %v8, i32* %slot1, align 8
  %v9 = load i32, i32* %slot2, align 8
  %v10 = add i32 0, 1
  %v11 = add i32 %v9, %v10
  store i32 %v11, i32* %slot2, align 8
  br label %bb1

bb3:
  %v12 = load i32, i32* %slot1, align 8
  %boxed0 = call i8* @dryad_box_i32(i32 %v12)
  ret i8* %boxed0

}

declare i8* @dryad_argv_box_i32(i8**, i32)
declare i8* @dryad_argv_box_f64(i8**, i32)
declare i8* @dryad_argv_box_bool(i8**, i32)
declare i8* @dryad_argv_box_cstring(i8**, i32)
define i32 @main(i32 %argc, i8** %argv) {
entry:
  %main_arg0 = call i8* @dryad_argv_box_cstring(i8** %argv, i32 1)
  call i8* @compute_sum(i8* %main_arg0)
  ret i32 0
}
