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

@dryad_lit___lambda_1_0 = private unnamed_addr constant [3 x i8] c"ok\00"
@dryad_lit___dryad_main_0 = private unnamed_addr constant [16 x i8] c"Promise_resolve\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [14 x i8] c"Promise_flush\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [8 x i8] c"resolve\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [5 x i8] c"then\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [12 x i8] c"Promise_all\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [4 x i8] c"all\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [6 x i8] c"first\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [7 x i8] c"second\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [13 x i8] c"Promise_race\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [5 x i8] c"race\00"
@dryad_lit___dryad_main_11 = private unnamed_addr constant [4 x i8] c"err\00"
@dryad_lit___dryad_main_12 = private unnamed_addr constant [15 x i8] c"Promise_reject\00"
@dryad_lit___dryad_main_13 = private unnamed_addr constant [6 x i8] c"catch\00"

define i8* @__lambda_0(i8* %arg0) {
  %slot0 = alloca i8*, align 8
  store i8* %arg0, i8** %slot0, align 8
  br label %bb0

bb0:
  %v0 = load i8*, i8** %slot0, align 8
  %v1 = add i32 0, 1
  %v2 = call i32 @dryad_unbox_i32(i8* %v0)
  %v3 = add i32 %v2, %v1
  %boxed0 = call i8* @dryad_box_i32(i32 %v3)
  ret i8* %boxed0

}

define i8* @__lambda_1(i8* %arg0) {
  %slot0 = alloca i8*, align 8
  store i8* %arg0, i8** %slot0, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @dryad_lit___lambda_1_0, i64 0, i64 0))
  ret i8* %v0

}

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  %slot2 = alloca i8*, align 8
  %slot3 = alloca i8*, align 8
  %slot4 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = add i32 0, 42
  %v1 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v1, i8** %slot0, align 8
  %v2 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v3 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  call void @dryad_print(i8* %v3)
  %v4 = inttoptr i64 0 to i8*
  %v5 = load i8*, i8** %slot0, align 8
  %v6 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v6)
  %v7 = inttoptr i64 0 to i8*
  %v8 = load i8*, i8** %slot0, align 8
  %v9 = inttoptr i64 0 to i8*
  %v10 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  store i8* %v10, i8** %slot1, align 8
  %v11 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v12 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  call void @dryad_print(i8* %v12)
  %v13 = inttoptr i64 0 to i8*
  %v14 = load i8*, i8** %slot1, align 8
  %v15 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v15)
  %v16 = inttoptr i64 0 to i8*
  %v17 = call i8* @dryad_new_array()
  %v18 = add i32 0, 10
  %v19 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %as_i8_0 = bitcast ptr %v19 to i8*
  %v20 = call i8* @dryad_array_push(i8* %v17, i8* %as_i8_0)
  %v21 = add i32 0, 20
  %v22 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %as_i8_1 = bitcast ptr %v22 to i8*
  %v23 = call i8* @dryad_array_push(i8* %v20, i8* %as_i8_1)
  %v24 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v24, i8** %slot2, align 8
  %v25 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v26 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  call void @dryad_print(i8* %v26)
  %v27 = inttoptr i64 0 to i8*
  %v28 = load i8*, i8** %slot2, align 8
  %v29 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  %v30 = add i32 0, 0
  %ge_idx31 = call i8* @dryad_box_i32(i32 %v30)
  %v31 = call i8* @dryad_variant_get_element(i8* %v29, i8* %ge_idx31)
  %v32 = call i32 @dryad_unbox_i32(i8* %v31)
  %print_box33 = call i8* @dryad_box_i32(i32 %v32)
  call void @dryad_print(i8* %print_box33)
  %v33 = inttoptr i64 0 to i8*
  %v34 = load i8*, i8** %slot2, align 8
  %v35 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  %v36 = add i32 0, 1
  %ge_idx37 = call i8* @dryad_box_i32(i32 %v36)
  %v37 = call i8* @dryad_variant_get_element(i8* %v35, i8* %ge_idx37)
  %v38 = call i32 @dryad_unbox_i32(i8* %v37)
  %print_box39 = call i8* @dryad_box_i32(i32 %v38)
  call void @dryad_print(i8* %print_box39)
  %v39 = inttoptr i64 0 to i8*
  %v40 = call i8* @dryad_new_array()
  %v41 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_7, i64 0, i64 0))
  %v42 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %as_i8_2 = bitcast ptr %v42 to i8*
  %v43 = call i8* @dryad_array_push(i8* %v40, i8* %as_i8_2)
  %v44 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @dryad_lit___dryad_main_8, i64 0, i64 0))
  %v45 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %as_i8_3 = bitcast ptr %v45 to i8*
  %v46 = call i8* @dryad_array_push(i8* %v43, i8* %as_i8_3)
  %v47 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v47, i8** %slot3, align 8
  %v48 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v49 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_10, i64 0, i64 0))
  call void @dryad_print(i8* %v49)
  %v50 = inttoptr i64 0 to i8*
  %v51 = load i8*, i8** %slot3, align 8
  %v52 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v52)
  %v53 = inttoptr i64 0 to i8*
  %v54 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_11, i64 0, i64 0))
  %v55 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v56 = inttoptr i64 0 to i8*
  %v57 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  store i8* %v57, i8** %slot4, align 8
  %v58 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v59 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_13, i64 0, i64 0))
  call void @dryad_print(i8* %v59)
  %v60 = inttoptr i64 0 to i8*
  %v61 = load i8*, i8** %slot4, align 8
  %v62 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v62)
  %v63 = inttoptr i64 0 to i8*
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
