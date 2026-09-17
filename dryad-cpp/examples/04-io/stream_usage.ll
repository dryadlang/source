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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [18 x i8] c"in-memory payload\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [24 x i8] c"MemoryStream_fromString\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [22 x i8] c"MemoryStream_toString\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [6 x i8] c"close\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [15 x i8] c"MemoryStream: \00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [5 x i8] c"Path\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [15 x i8] c"dryad_examples\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [16 x i8] c"stream_copy.txt\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [5 x i8] c"join\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [21 x i8] c"FileStream copy demo\00"
@dryad_lit___dryad_main_11 = private unnamed_addr constant [18 x i8] c"File_writeAllText\00"
@dryad_lit___dryad_main_12 = private unnamed_addr constant [9 x i8] c"FileMode\00"
@dryad_lit___dryad_main_13 = private unnamed_addr constant [10 x i8] c"File_open\00"
@dryad_lit___dryad_main_14 = private unnamed_addr constant [20 x i8] c"stream_copy_out.txt\00"
@dryad_lit___dryad_main_15 = private unnamed_addr constant [11 x i8] c"Buffer_New\00"
@dryad_lit___dryad_main_16 = private unnamed_addr constant [5 x i8] c"read\00"
@dryad_lit___dryad_main_17 = private unnamed_addr constant [6 x i8] c"write\00"
@dryad_lit___dryad_main_18 = private unnamed_addr constant [15 x i8] c"Buffer_Dispose\00"
@dryad_lit___dryad_main_19 = private unnamed_addr constant [17 x i8] c"File_readAllText\00"
@dryad_lit___dryad_main_20 = private unnamed_addr constant [9 x i8] c"Copied: \00"
@dryad_lit___dryad_main_21 = private unnamed_addr constant [12 x i8] c"File_delete\00"

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  %slot2 = alloca i8*, align 8
  %slot3 = alloca i8*, align 8
  %slot4 = alloca i8*, align 8
  %slot5 = alloca i8*, align 8
  %slot6 = alloca i8*, align 8
  %slot7 = alloca i8*, align 8
  %slot8 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([18 x i8], [18 x i8]* @dryad_lit___dryad_main_0, i64 0, i64 0))
  %v1 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v1, i8** %slot0, align 8
  %v2 = load i8*, i8** %slot0, align 8
  %v3 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v3, i8** %slot1, align 8
  %v4 = load i8*, i8** %slot0, align 8
  %v5 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
  %v6 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  %v7 = load i8*, i8** %slot1, align 8
  %ga_ul8 = call i32 @dryad_unbox_i32(i8* %v6)
  %ga_ur8 = call i32 @dryad_unbox_i32(i8* %v7)
  %ga_l8 = call i8* @dryad_box_i32(i32 %ga_ul8)
  %ga_r8 = call i8* @dryad_box_i32(i32 %ga_ur8)
  %v8 = call i8* @dryad_variant_add(i8* %ga_l8, i8* %ga_r8)
  call void @dryad_print(i8* %v8)
  %v9 = inttoptr i64 0 to i8*
  %v10 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v11 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @dryad_lit___dryad_main_7, i64 0, i64 0))
  %v12 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([16 x i8], [16 x i8]* @dryad_lit___dryad_main_8, i64 0, i64 0))
  %v13 = call i8* @dryad_intrinsic_slow(i16 65415, i8 2)
  store i8* %v13, i8** %slot2, align 8
  %v14 = load i8*, i8** %slot2, align 8
  %v15 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([21 x i8], [21 x i8]* @dryad_lit___dryad_main_10, i64 0, i64 0))
  %v16 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  %v17 = load i8*, i8** %slot2, align 8
  %v18 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v19 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  %v20 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  store i8* %v20, i8** %slot3, align 8
  %v21 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v22 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @dryad_lit___dryad_main_7, i64 0, i64 0))
  %v23 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([20 x i8], [20 x i8]* @dryad_lit___dryad_main_14, i64 0, i64 0))
  %v24 = call i8* @dryad_intrinsic_slow(i16 65415, i8 2)
  store i8* %v24, i8** %slot4, align 8
  %v25 = load i8*, i8** %slot4, align 8
  %v26 = call i8* @dryad_intrinsic_slow(i16 65378, i8 0)
  %v27 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  %v28 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  store i8* %v28, i8** %slot5, align 8
  %v29 = add i32 0, 256
  %v30 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v30, i8** %slot6, align 8
  %v31 = load i8*, i8** %slot3, align 8
  %v32 = load i8*, i8** %slot6, align 8
  %v33 = add i32 0, 0
  %v34 = add i32 0, 256
  %v35 = call i8* @dryad_intrinsic_slow(i16 65415, i8 3)
  store i8* %v35, i8** %slot7, align 8
  %v36 = load i8*, i8** %slot5, align 8
  %v37 = load i8*, i8** %slot6, align 8
  %v38 = add i32 0, 0
  %slot_load0 = load i8*, i8** %slot7, align 8
  %v39 = call i32 @dryad_unbox_i32(i8* %slot_load0)
  %v40 = call i8* @dryad_intrinsic_slow(i16 65415, i8 3)
  %v41 = load i8*, i8** %slot6, align 8
  %v42 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v43 = load i8*, i8** %slot3, align 8
  %v44 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
  %v45 = load i8*, i8** %slot5, align 8
  %v46 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
  %v47 = load i8*, i8** %slot4, align 8
  %v48 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v48, i8** %slot8, align 8
  %v49 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_20, i64 0, i64 0))
  %v50 = load i8*, i8** %slot8, align 8
  %ga_ul51 = call i32 @dryad_unbox_i32(i8* %v49)
  %ga_ur51 = call i32 @dryad_unbox_i32(i8* %v50)
  %ga_l51 = call i8* @dryad_box_i32(i32 %ga_ul51)
  %ga_r51 = call i8* @dryad_box_i32(i32 %ga_ur51)
  %v51 = call i8* @dryad_variant_add(i8* %ga_l51, i8* %ga_r51)
  call void @dryad_print(i8* %v51)
  %v52 = inttoptr i64 0 to i8*
  %v53 = load i8*, i8** %slot2, align 8
  %v54 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v55 = load i8*, i8** %slot4, align 8
  %v56 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
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
