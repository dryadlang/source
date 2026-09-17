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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [5 x i8] c"Path\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [4 x i8] c"foo\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [4 x i8] c"bar\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [5 x i8] c"join\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [8 x i8] c"baz.txt\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [8 x i8] c"dirname\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [14 x i8] c"/tmp/test.txt\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [9 x i8] c"basename\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [10 x i8] c"extension\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [9 x i8] c"test.txt\00"

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v1 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_1, i64 0, i64 0))
  %v2 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  %v3 = call i8* @dryad_intrinsic_slow(i16 65415, i8 2)
  store i8* %v3, i8** %slot0, align 8
  %v4 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v5 = load i8*, i8** %slot0, align 8
  %v6 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  %v7 = call i8* @dryad_intrinsic_slow(i16 65415, i8 2)
  store i8* %v7, i8** %slot0, align 8
  %v8 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_3, i64 0, i64 0))
  call void @dryad_print(i8* %v8)
  %v9 = inttoptr i64 0 to i8*
  %v10 = load i8*, i8** %slot0, align 8
  call void @dryad_print(i8* %v10)
  %v11 = inttoptr i64 0 to i8*
  %v12 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  call void @dryad_print(i8* %v12)
  %v13 = inttoptr i64 0 to i8*
  %v14 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v15 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @dryad_lit___dryad_main_7, i64 0, i64 0))
  %v16 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  call void @dryad_print(i8* %v16)
  %v17 = inttoptr i64 0 to i8*
  %v18 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_8, i64 0, i64 0))
  call void @dryad_print(i8* %v18)
  %v19 = inttoptr i64 0 to i8*
  %v20 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v21 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @dryad_lit___dryad_main_7, i64 0, i64 0))
  %v22 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  call void @dryad_print(i8* %v22)
  %v23 = inttoptr i64 0 to i8*
  %v24 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @dryad_lit___dryad_main_9, i64 0, i64 0))
  call void @dryad_print(i8* %v24)
  %v25 = inttoptr i64 0 to i8*
  %v26 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v27 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_10, i64 0, i64 0))
  %v28 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  call void @dryad_print(i8* %v28)
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
