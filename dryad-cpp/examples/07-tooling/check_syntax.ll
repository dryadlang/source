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

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = add i32 0, 1
  %v1 = add i32 0, 2
  %v2 = call i8* @dryad_intrinsic_slow(i16 65280, i8 2)
  store i8* %v2, i8** %slot0, align 8
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
