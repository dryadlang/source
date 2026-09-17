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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [4 x i8] c"Rex\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [9 x i8] c"Labrador\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [4 x i8] c"Dog\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [5 x i8] c"name\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [6 x i8] c"breed\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [6 x i8] c"speak\00"

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_0, i64 0, i64 0))
  %v1 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_1, i64 0, i64 0))
  %v2 = call i8* @dryad_intrinsic_slow(i16 65416, i8 2)
  store i8* %v2, i8** %slot0, align 8
  %v3 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_3, i64 0, i64 0))
  call void @dryad_print(i8* %v3)
  %v4 = inttoptr i64 0 to i8*
  %v5 = load i8*, i8** %slot0, align 8
  %v6 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v6)
  %v7 = inttoptr i64 0 to i8*
  %v8 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_5, i64 0, i64 0))
  call void @dryad_print(i8* %v8)
  %v9 = inttoptr i64 0 to i8*
  %v10 = load i8*, i8** %slot0, align 8
  %v11 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v11)
  %v12 = inttoptr i64 0 to i8*
  %v13 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  call void @dryad_print(i8* %v13)
  %v14 = inttoptr i64 0 to i8*
  %v15 = load i8*, i8** %slot0, align 8
  %v16 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
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
