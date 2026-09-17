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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [1 x i8] c"\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [2 x i8] c"A\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [2 x i8] c"B\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [2 x i8] c"C\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [6 x i8] c"grade\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [9 x i8] c"sum 1..5\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [12 x i8] c"array total\00"

define i8* @__dryad_main() {
  %slot0 = alloca i32, align 8
  %slot1 = alloca i8*, align 8
  %slot2 = alloca i32, align 8
  %slot3 = alloca i32, align 8
  %slot4 = alloca i8*, align 8
  %slot5 = alloca i32, align 8
  %slot6 = alloca i32, align 8
  br label %bb0

bb0:
  %v0 = add i32 0, 85
  store i32 %v0, i32* %slot0, align 8
  %v1 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([1 x i8], [1 x i8]* @dryad_lit___dryad_main_0, i64 0, i64 0))
  store i8* %v1, i8** %slot1, align 8
  %v2 = load i32, i32* %slot0, align 8
  %v3 = add i32 0, 90
  %v4 = icmp sge i32 %v2, %v3
  br i1 %v4, label %bb1, label %bb2

bb1:
  %v5 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @dryad_lit___dryad_main_1, i64 0, i64 0))
  store i8* %v5, i8** %slot1, align 8
  br label %bb5

bb2:
  %v6 = load i32, i32* %slot0, align 8
  %v7 = add i32 0, 70
  %v8 = icmp sge i32 %v6, %v7
  br i1 %v8, label %bb3, label %bb4

bb3:
  %v9 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  store i8* %v9, i8** %slot1, align 8
  br label %bb5

bb4:
  %v10 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([2 x i8], [2 x i8]* @dryad_lit___dryad_main_3, i64 0, i64 0))
  store i8* %v10, i8** %slot1, align 8
  br label %bb5

bb5:
  %v11 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  call void @dryad_print(i8* %v11)
  %v12 = inttoptr i64 0 to i8*
  %v13 = load i8*, i8** %slot1, align 8
  call void @dryad_print(i8* %v13)
  %v14 = inttoptr i64 0 to i8*
  %v15 = add i32 0, 0
  store i32 %v15, i32* %slot2, align 8
  %v16 = add i32 0, 1
  store i32 %v16, i32* %slot3, align 8
  br label %bb6

bb6:
  %v17 = load i32, i32* %slot3, align 8
  %v18 = add i32 0, 5
  %v19 = icmp sle i32 %v17, %v18
  br i1 %v19, label %bb7, label %bb8

bb7:
  %v20 = load i32, i32* %slot2, align 8
  %v21 = load i32, i32* %slot3, align 8
  %v22 = add i32 %v20, %v21
  store i32 %v22, i32* %slot2, align 8
  %v23 = load i32, i32* %slot3, align 8
  %v24 = add i32 0, 1
  %v25 = add i32 %v23, %v24
  store i32 %v25, i32* %slot3, align 8
  br label %bb6

bb8:
  %v26 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  call void @dryad_print(i8* %v26)
  %v27 = inttoptr i64 0 to i8*
  %v28 = load i32, i32* %slot2, align 8
  %print_box29 = call i8* @dryad_box_i32(i32 %v28)
  call void @dryad_print(i8* %print_box29)
  %v29 = inttoptr i64 0 to i8*
  %v30 = call i8* @dryad_new_array()
  %v31 = add i32 0, 10
  %apush_box32 = call i8* @dryad_box_i32(i32 %v31)
  %v32 = call i8* @dryad_array_push(i8* %v30, i8* %apush_box32)
  %v33 = add i32 0, 20
  %apush_box34 = call i8* @dryad_box_i32(i32 %v33)
  %v34 = call i8* @dryad_array_push(i8* %v32, i8* %apush_box34)
  %v35 = add i32 0, 30
  %apush_box36 = call i8* @dryad_box_i32(i32 %v35)
  %v36 = call i8* @dryad_array_push(i8* %v34, i8* %apush_box36)
  store i8* %v36, i8** %slot4, align 8
  %v37 = add i32 0, 0
  store i32 %v37, i32* %slot5, align 8
  %v38 = add i32 0, 0
  store i32 %v38, i32* %slot6, align 8
  br label %bb9

bb9:
  %v39 = load i32, i32* %slot6, align 8
  %v40 = add i32 0, 3
  %v41 = icmp slt i32 %v39, %v40
  br i1 %v41, label %bb10, label %bb11

bb10:
  %v42 = load i32, i32* %slot5, align 8
  %v43 = load i8*, i8** %slot4, align 8
  %v44 = load i32, i32* %slot6, align 8
  %ge_idx45 = call i8* @dryad_box_i32(i32 %v44)
  %v45 = call i8* @dryad_variant_get_element(i8* %v43, i8* %ge_idx45)
  %v46 = call i32 @dryad_unbox_i32(i8* %v45)
  %v47 = add i32 %v42, %v46
  store i32 %v47, i32* %slot5, align 8
  %v48 = load i32, i32* %slot6, align 8
  %v49 = add i32 0, 1
  %v50 = add i32 %v48, %v49
  store i32 %v50, i32* %slot6, align 8
  br label %bb9

bb11:
  %v51 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_7, i64 0, i64 0))
  call void @dryad_print(i8* %v51)
  %v52 = inttoptr i64 0 to i8*
  %v53 = load i32, i32* %slot5, align 8
  %print_box54 = call i8* @dryad_box_i32(i32 %v53)
  call void @dryad_print(i8* %print_box54)
  %v54 = inttoptr i64 0 to i8*
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
