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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [5 x i8] c"List\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [4 x i8] c"add\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [4 x i8] c"map\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [7 x i8] c"filter\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [7 x i8] c"reduce\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [11 x i8] c"List count\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [14 x i8] c"Doubled first\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [4 x i8] c"get\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [12 x i8] c"Evens count\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [4 x i8] c"Sum\00"
@dryad_lit___dryad_main_11 = private unnamed_addr constant [4 x i8] c"Map\00"
@dryad_lit___dryad_main_12 = private unnamed_addr constant [5 x i8] c"lang\00"
@dryad_lit___dryad_main_13 = private unnamed_addr constant [6 x i8] c"Dryad\00"
@dryad_lit___dryad_main_14 = private unnamed_addr constant [4 x i8] c"put\00"
@dryad_lit___dryad_main_15 = private unnamed_addr constant [6 x i8] c"phase\00"
@dryad_lit___dryad_main_16 = private unnamed_addr constant [9 x i8] c"Map size\00"
@dryad_lit___dryad_main_17 = private unnamed_addr constant [5 x i8] c"Lang\00"
@dryad_lit___dryad_main_18 = private unnamed_addr constant [4 x i8] c"Set\00"
@dryad_lit___dryad_main_19 = private unnamed_addr constant [7 x i8] c"stdlib\00"
@dryad_lit___dryad_main_20 = private unnamed_addr constant [12 x i8] c"collections\00"
@dryad_lit___dryad_main_21 = private unnamed_addr constant [9 x i8] c"Set size\00"
@dryad_lit___dryad_main_22 = private unnamed_addr constant [16 x i8] c"Has collections\00"
@dryad_lit___dryad_main_23 = private unnamed_addr constant [4 x i8] c"has\00"
@dryad_lit___dryad_main_24 = private unnamed_addr constant [6 x i8] c"Queue\00"
@dryad_lit___dryad_main_25 = private unnamed_addr constant [8 x i8] c"enqueue\00"
@dryad_lit___dryad_main_26 = private unnamed_addr constant [11 x i8] c"Queue peek\00"
@dryad_lit___dryad_main_27 = private unnamed_addr constant [5 x i8] c"peek\00"
@dryad_lit___dryad_main_28 = private unnamed_addr constant [14 x i8] c"Queue dequeue\00"
@dryad_lit___dryad_main_29 = private unnamed_addr constant [8 x i8] c"dequeue\00"
@dryad_lit___dryad_main_30 = private unnamed_addr constant [12 x i8] c"Queue count\00"

define i8* @__lambda_0(i8* %arg0) {
  %slot0 = alloca i8*, align 8
  store i8* %arg0, i8** %slot0, align 8
  br label %bb0

bb0:
  %v0 = load i8*, i8** %slot0, align 8
  %v1 = add i32 0, 2
  %v2 = call i8* @dryad_intrinsic_slow(i16 65298, i8 2)
  ret i8* %v2

}

define i8* @__lambda_1(i8* %arg0) {
  %slot0 = alloca i8*, align 8
  store i8* %arg0, i8** %slot0, align 8
  br label %bb0

bb0:
  %v0 = load i8*, i8** %slot0, align 8
  %v1 = add i32 0, 2
  %v2 = call i8* @dryad_intrinsic_slow(i16 65300, i8 2)
  %v3 = add i32 0, 0
  %v4 = call i32 @dryad_unbox_i32(i8* %v2)
  %v5 = icmp eq i32 %v4, %v3
  %boxed0 = call i8* @dryad_box_bool(i1 %v5)
  ret i8* %boxed0

}

define i8* @__lambda_2(i8* %arg0, i8* %arg1) {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  store i8* %arg0, i8** %slot0, align 8
  store i8* %arg1, i8** %slot1, align 8
  br label %bb0

bb0:
  %v0 = load i8*, i8** %slot0, align 8
  %v1 = load i8*, i8** %slot1, align 8
  %ga_ul2 = call i32 @dryad_unbox_i32(i8* %v0)
  %ga_ur2 = call i32 @dryad_unbox_i32(i8* %v1)
  %ga_l2 = call i8* @dryad_box_i32(i32 %ga_ul2)
  %ga_r2 = call i8* @dryad_box_i32(i32 %ga_ur2)
  %v2 = call i8* @dryad_variant_add(i8* %ga_l2, i8* %ga_r2)
  ret i8* %v2

}

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  %slot2 = alloca i8*, align 8
  %slot3 = alloca i8*, align 8
  %slot4 = alloca i8*, align 8
  %slot5 = alloca i8*, align 8
  %slot6 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_intrinsic_slow(i16 65416, i8 0)
  store i8* %v0, i8** %slot0, align 8
  %v1 = load i8*, i8** %slot0, align 8
  %v2 = add i32 0, 1
  %v3 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v4 = load i8*, i8** %slot0, align 8
  %v5 = add i32 0, 2
  %v6 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v7 = load i8*, i8** %slot0, align 8
  %v8 = add i32 0, 3
  %v9 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v10 = load i8*, i8** %slot0, align 8
  %v11 = add i32 0, 4
  %v12 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v13 = load i8*, i8** %slot0, align 8
  %v14 = inttoptr i64 0 to i8*
  %v15 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  store i8* %v15, i8** %slot1, align 8
  %v16 = load i8*, i8** %slot0, align 8
  %v17 = inttoptr i64 0 to i8*
  %v18 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  store i8* %v18, i8** %slot2, align 8
  %v19 = load i8*, i8** %slot0, align 8
  %v20 = inttoptr i64 0 to i8*
  %v21 = add i32 0, 0
  %v22 = call i8* @dryad_intrinsic_slow(i16 65415, i8 2)
  store i8* %v22, i8** %slot3, align 8
  %v23 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([11 x i8], [11 x i8]* @dryad_lit___dryad_main_5, i64 0, i64 0))
  call void @dryad_print(i8* %v23)
  %v24 = inttoptr i64 0 to i8*
  %v25 = load i8*, i8** %slot0, align 8
  %v26 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v26)
  %v27 = inttoptr i64 0 to i8*
  %v28 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @dryad_lit___dryad_main_7, i64 0, i64 0))
  call void @dryad_print(i8* %v28)
  %v29 = inttoptr i64 0 to i8*
  %v30 = load i8*, i8** %slot1, align 8
  %v31 = add i32 0, 0
  %v32 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  call void @dryad_print(i8* %v32)
  %v33 = inttoptr i64 0 to i8*
  %v34 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_9, i64 0, i64 0))
  call void @dryad_print(i8* %v34)
  %v35 = inttoptr i64 0 to i8*
  %v36 = load i8*, i8** %slot2, align 8
  %v37 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v37)
  %v38 = inttoptr i64 0 to i8*
  %v39 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_10, i64 0, i64 0))
  call void @dryad_print(i8* %v39)
  %v40 = inttoptr i64 0 to i8*
  %slot_load0 = load i8*, i8** %slot3, align 8
  %v41 = call i32 @dryad_unbox_i32(i8* %slot_load0)
  %print_box42 = call i8* @dryad_box_i32(i32 %v41)
  call void @dryad_print(i8* %print_box42)
  %v42 = inttoptr i64 0 to i8*
  %v43 = call i8* @dryad_intrinsic_slow(i16 65416, i8 0)
  store i8* %v43, i8** %slot4, align 8
  %v44 = load i8*, i8** %slot4, align 8
  %v45 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_12, i64 0, i64 0))
  %v46 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_13, i64 0, i64 0))
  %v47 = call i8* @dryad_intrinsic_slow(i16 65415, i8 2)
  %v48 = load i8*, i8** %slot4, align 8
  %v49 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_15, i64 0, i64 0))
  %v50 = add i32 0, 4
  %v51 = call i8* @dryad_intrinsic_slow(i16 65415, i8 2)
  %v52 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_16, i64 0, i64 0))
  call void @dryad_print(i8* %v52)
  %v53 = inttoptr i64 0 to i8*
  %v54 = load i8*, i8** %slot4, align 8
  %v55 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v55)
  %v56 = inttoptr i64 0 to i8*
  %v57 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_17, i64 0, i64 0))
  call void @dryad_print(i8* %v57)
  %v58 = inttoptr i64 0 to i8*
  %v59 = load i8*, i8** %slot4, align 8
  %v60 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_12, i64 0, i64 0))
  %v61 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  call void @dryad_print(i8* %v61)
  %v62 = inttoptr i64 0 to i8*
  %v63 = call i8* @dryad_intrinsic_slow(i16 65416, i8 0)
  store i8* %v63, i8** %slot5, align 8
  %v64 = load i8*, i8** %slot5, align 8
  %v65 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @dryad_lit___dryad_main_19, i64 0, i64 0))
  %v66 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v67 = load i8*, i8** %slot5, align 8
  %v68 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_20, i64 0, i64 0))
  %v69 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v70 = load i8*, i8** %slot5, align 8
  %v71 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @dryad_lit___dryad_main_19, i64 0, i64 0))
  %v72 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v73 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_21, i64 0, i64 0))
  call void @dryad_print(i8* %v73)
  %v74 = inttoptr i64 0 to i8*
  %v75 = load i8*, i8** %slot5, align 8
  %v76 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v76)
  %v77 = inttoptr i64 0 to i8*
  %v78 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @dryad_lit___dryad_main_22, i64 0, i64 0))
  call void @dryad_print(i8* %v78)
  %v79 = inttoptr i64 0 to i8*
  %v80 = load i8*, i8** %slot5, align 8
  %v81 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_20, i64 0, i64 0))
  %v82 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  call void @dryad_print(i8* %v82)
  %v83 = inttoptr i64 0 to i8*
  %v84 = add i32 0, 4
  %v85 = call i8* @dryad_intrinsic_slow(i16 65416, i8 1)
  store i8* %v85, i8** %slot6, align 8
  %v86 = load i8*, i8** %slot6, align 8
  %v87 = add i32 0, 10
  %v88 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v89 = load i8*, i8** %slot6, align 8
  %v90 = add i32 0, 20
  %v91 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v92 = load i8*, i8** %slot6, align 8
  %v93 = add i32 0, 30
  %v94 = call i8* @dryad_intrinsic_slow(i16 65415, i8 1)
  %v95 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([11 x i8], [11 x i8]* @dryad_lit___dryad_main_26, i64 0, i64 0))
  call void @dryad_print(i8* %v95)
  %v96 = inttoptr i64 0 to i8*
  %v97 = load i8*, i8** %slot6, align 8
  %v98 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
  call void @dryad_print(i8* %v98)
  %v99 = inttoptr i64 0 to i8*
  %v100 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @dryad_lit___dryad_main_28, i64 0, i64 0))
  call void @dryad_print(i8* %v100)
  %v101 = inttoptr i64 0 to i8*
  %v102 = load i8*, i8** %slot6, align 8
  %v103 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
  call void @dryad_print(i8* %v103)
  %v104 = inttoptr i64 0 to i8*
  %v105 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_30, i64 0, i64 0))
  call void @dryad_print(i8* %v105)
  %v106 = inttoptr i64 0 to i8*
  %v107 = load i8*, i8** %slot6, align 8
  %v108 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v108)
  %v109 = inttoptr i64 0 to i8*
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
