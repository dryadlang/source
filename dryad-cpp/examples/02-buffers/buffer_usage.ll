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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [13 x i8] c"Olá, mundo!\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [18 x i8] c"Buffer_FromString\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [16 x i8] c"Buffer_AsString\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [4 x i8] c"len\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [11 x i8] c"Buffer_Len\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [10 x i8] c"as_string\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [13 x i8] c"Buffer_Slice\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [6 x i8] c"slice\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [5 x i8] c"Olá\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [15 x i8] c"Buffer_Dispose\00"
@dryad_lit___dryad_main_11 = private unnamed_addr constant [13 x i8] c"Buffer_Write\00"
@dryad_lit___dryad_main_12 = private unnamed_addr constant [9 x i8] c"modified\00"
@dryad_lit___dryad_main_13 = private unnamed_addr constant [13 x i8] c"Xlá, mundo!\00"
@dryad_lit___dryad_main_14 = private unnamed_addr constant [9 x i8] c"disposed\00"

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  %slot2 = alloca i1, align 8
  %slot3 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @dryad_lit___dryad_main_0, i64 0, i64 0))
  store i8* %v0, i8** %slot0, align 8
  %v1 = load i8*, i8** %slot0, align 8
  %v2 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v2, i8** %slot1, align 8
  %v3 = load i8*, i8** %slot1, align 8
  %v4 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v5 = load i8*, i8** %slot0, align 8
  %as_i8_0 = bitcast ptr %v4 to i8*
  %as_i8_1 = bitcast ptr %v5 to i8*
  %v6 = call i1 @dryad_variant_eq(i8* %as_i8_0, i8* %as_i8_1)
  store i1 %v6, i1* %slot2, align 8
  %v7 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_3, i64 0, i64 0))
  call void @dryad_print(i8* %v7)
  %v8 = inttoptr i64 0 to i8*
  %v9 = load i8*, i8** %slot1, align 8
  %v10 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  call void @dryad_print(i8* %v10)
  %v11 = inttoptr i64 0 to i8*
  %v12 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  call void @dryad_print(i8* %v12)
  %v13 = inttoptr i64 0 to i8*
  %v14 = load i8*, i8** %slot1, align 8
  %v15 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  call void @dryad_print(i8* %v15)
  %v16 = inttoptr i64 0 to i8*
  %v17 = load i8*, i8** %slot1, align 8
  %v18 = add i32 0, 0
  %v19 = add i32 0, 4
  %v20 = call i8* @dryad_intrinsic_slow(i16 65417, i8 3)
  store i8* %v20, i8** %slot3, align 8
  %v21 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_8, i64 0, i64 0))
  call void @dryad_print(i8* %v21)
  %v22 = inttoptr i64 0 to i8*
  %v23 = load i8*, i8** %slot3, align 8
  %v24 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  call void @dryad_print(i8* %v24)
  %v25 = inttoptr i64 0 to i8*
  %v26 = load i1, i1* %slot2, align 8
  br i1 %v26, label %bb1, label %bb2

bb1:
  %v27 = load i8*, i8** %slot3, align 8
  %v28 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v29 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_9, i64 0, i64 0))
  %as_i8_2 = bitcast ptr %v28 to i8*
  %as_i8_3 = bitcast ptr %v29 to i8*
  %v30 = call i1 @dryad_variant_eq(i8* %as_i8_2, i8* %as_i8_3)
  br label %bb2

bb2:
  %v31 = phi i1 [%v26, %bb0], [%v30, %bb1]
  store i1 %v31, i1* %slot2, align 8
  %v32 = load i8*, i8** %slot3, align 8
  %v33 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v34 = load i8*, i8** %slot1, align 8
  %v35 = add i32 0, 0
  %v36 = add i32 0, 88
  %v37 = call i8* @dryad_intrinsic_slow(i16 65417, i8 3)
  %v38 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_12, i64 0, i64 0))
  call void @dryad_print(i8* %v38)
  %v39 = inttoptr i64 0 to i8*
  %v40 = load i8*, i8** %slot1, align 8
  %v41 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  call void @dryad_print(i8* %v41)
  %v42 = inttoptr i64 0 to i8*
  %v43 = load i1, i1* %slot2, align 8
  br i1 %v43, label %bb3, label %bb4

bb3:
  %v44 = load i8*, i8** %slot1, align 8
  %v45 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v46 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([13 x i8], [13 x i8]* @dryad_lit___dryad_main_13, i64 0, i64 0))
  %as_i8_4 = bitcast ptr %v45 to i8*
  %as_i8_5 = bitcast ptr %v46 to i8*
  %v47 = call i1 @dryad_variant_eq(i8* %as_i8_4, i8* %as_i8_5)
  br label %bb4

bb4:
  %v48 = phi i1 [%v43, %bb2], [%v47, %bb3]
  store i1 %v48, i1* %slot2, align 8
  %v49 = load i8*, i8** %slot1, align 8
  %v50 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v51 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_14, i64 0, i64 0))
  call void @dryad_print(i8* %v51)
  %v52 = inttoptr i64 0 to i8*
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
