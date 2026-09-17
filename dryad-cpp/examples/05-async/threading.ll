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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [11 x i8] c"worker_add\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [7 x i8] c"Thread\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [6 x i8] c"start\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [5 x i8] c"join\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [12 x i8] c"thread join\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [6 x i8] c"value\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [16 x i8] c"worker_identity\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [16 x i8] c"Thread_parallel\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [9 x i8] c"parallel\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [11 x i8] c"ThreadPool\00"
@dryad_lit___dryad_main_11 = private unnamed_addr constant [4 x i8] c"run\00"
@dryad_lit___dryad_main_12 = private unnamed_addr constant [9 x i8] c"pool run\00"
@dryad_lit___dryad_main_13 = private unnamed_addr constant [15 x i8] c"ThreadPool_map\00"
@dryad_lit___dryad_main_14 = private unnamed_addr constant [8 x i8] c"destroy\00"
@dryad_lit___dryad_main_15 = private unnamed_addr constant [9 x i8] c"pool map\00"

define i8* @worker_add(i8* %arg0, i8* %arg1) {
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

define i8* @worker_identity() {
  br label %bb0

bb0:
  %v0 = add i32 0, 99
  %boxed0 = call i8* @dryad_box_i32(i32 %v0)
  ret i8* %boxed0

}

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  %slot2 = alloca i8*, align 8
  %slot3 = alloca i8*, align 8
  %slot4 = alloca i8*, align 8
  %slot5 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v1 = call i8* @dryad_new_array()
  %v2 = add i32 0, 20
  %apush_box3 = call i8* @dryad_box_i32(i32 %v2)
  %v3 = call i8* @dryad_array_push(i8* %v1, i8* %apush_box3)
  %v4 = add i32 0, 22
  %apush_box5 = call i8* @dryad_box_i32(i32 %v4)
  %v5 = call i8* @dryad_array_push(i8* %v3, i8* %apush_box5)
  %v6 = call i8* @dryad_intrinsic_slow(i16 65416, i8 2)
  store i8* %v6, i8** %slot0, align 8
  %v7 = load i8*, i8** %slot0, align 8
  %v8 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
  %v9 = load i8*, i8** %slot0, align 8
  %v10 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
  store i8* %v10, i8** %slot1, align 8
  %v11 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  call void @dryad_print(i8* %v11)
  %v12 = inttoptr i64 0 to i8*
  %v13 = load i8*, i8** %slot1, align 8
  %v14 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  %v15 = call i8* @dryad_intrinsic_slow(i16 615, i8 2)
  call void @dryad_print(i8* %v15)
  %v16 = inttoptr i64 0 to i8*
  %v17 = call i8* @dryad_new_array()
  %v18 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %as_i8_0 = bitcast ptr %v18 to i8*
  %v19 = call i8* @dryad_array_push(i8* %v17, i8* %as_i8_0)
  %v20 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %as_i8_1 = bitcast ptr %v20 to i8*
  %v21 = call i8* @dryad_array_push(i8* %v19, i8* %as_i8_1)
  %v22 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v22, i8** %slot2, align 8
  %v23 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_9, i64 0, i64 0))
  call void @dryad_print(i8* %v23)
  %v24 = inttoptr i64 0 to i8*
  %v25 = load i8*, i8** %slot2, align 8
  %v26 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  %v27 = call i8* @dryad_intrinsic_slow(i16 615, i8 2)
  %v28 = add i32 0, 0
  %ge_idx29 = call i8* @dryad_box_i32(i32 %v28)
  %v29 = call i8* @dryad_variant_get_element(i8* %v27, i8* %ge_idx29)
  %v30 = call i32 @dryad_unbox_i32(i8* %v29)
  %print_box31 = call i8* @dryad_box_i32(i32 %v30)
  call void @dryad_print(i8* %print_box31)
  %v31 = inttoptr i64 0 to i8*
  %v32 = add i32 0, 2
  %v33 = call i8* @dryad_intrinsic_slow(i16 65416, i8 1)
  store i8* %v33, i8** %slot3, align 8
  %v34 = load i8*, i8** %slot3, align 8
  %v35 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v36 = call i8* @dryad_new_array()
  %v37 = add i32 0, 10
  %apush_box38 = call i8* @dryad_box_i32(i32 %v37)
  %v38 = call i8* @dryad_array_push(i8* %v36, i8* %apush_box38)
  %v39 = add i32 0, 32
  %apush_box40 = call i8* @dryad_box_i32(i32 %v39)
  %v40 = call i8* @dryad_array_push(i8* %v38, i8* %apush_box40)
  %v41 = call i8* @dryad_intrinsic_slow(i16 65415, i8 2)
  store i8* %v41, i8** %slot4, align 8
  %v42 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_12, i64 0, i64 0))
  call void @dryad_print(i8* %v42)
  %v43 = inttoptr i64 0 to i8*
  %slot_load2 = load i8*, i8** %slot4, align 8
  %v44 = call i32 @dryad_unbox_i32(i8* %slot_load2)
  %print_box45 = call i8* @dryad_box_i32(i32 %v44)
  call void @dryad_print(i8* %print_box45)
  %v45 = inttoptr i64 0 to i8*
  %v46 = load i8*, i8** %slot3, align 8
  %v47 = call i8* @dryad_new_array()
  %v48 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %as_i8_3 = bitcast ptr %v48 to i8*
  %v49 = call i8* @dryad_array_push(i8* %v47, i8* %as_i8_3)
  %v50 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %as_i8_4 = bitcast ptr %v50 to i8*
  %v51 = call i8* @dryad_array_push(i8* %v49, i8* %as_i8_4)
  %v52 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  store i8* %v52, i8** %slot5, align 8
  %v53 = load i8*, i8** %slot3, align 8
  %v54 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
  %v55 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_15, i64 0, i64 0))
  call void @dryad_print(i8* %v55)
  %v56 = inttoptr i64 0 to i8*
  %v57 = load i8*, i8** %slot5, align 8
  %v58 = add i32 0, 0
  %ge_idx59 = call i8* @dryad_box_i32(i32 %v58)
  %v59 = call i8* @dryad_variant_get_element(i8* %v57, i8* %ge_idx59)
  %v60 = call i32 @dryad_unbox_i32(i8* %v59)
  %print_box61 = call i8* @dryad_box_i32(i32 %v60)
  call void @dryad_print(i8* %print_box61)
  %v61 = inttoptr i64 0 to i8*
  %v62 = load i8*, i8** %slot5, align 8
  %v63 = add i32 0, 1
  %ge_idx64 = call i8* @dryad_box_i32(i32 %v63)
  %v64 = call i8* @dryad_variant_get_element(i8* %v62, i8* %ge_idx64)
  %v65 = call i32 @dryad_unbox_i32(i8* %v64)
  %print_box66 = call i8* @dryad_box_i32(i32 %v65)
  call void @dryad_print(i8* %print_box66)
  %v66 = inttoptr i64 0 to i8*
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
