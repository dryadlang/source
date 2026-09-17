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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [14 x i8] c"MemoryBackend\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [4 x i8] c"VFS\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [10 x i8] c"/demo.txt\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [15 x i8] c"Hello from VFS\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [14 x i8] c"VFS_writeFile\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [13 x i8] c"VFS_readFile\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [15 x i8] c"VFS_removeFile\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [14 x i8] c"setVFSBackend\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [9 x i8] c"/api.txt\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [20 x i8] c"Hello from file API\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [10 x i8] c"writeFile\00"
@dryad_lit___dryad_main_11 = private unnamed_addr constant [9 x i8] c"readFile\00"
@dryad_lit___dryad_main_12 = private unnamed_addr constant [11 x i8] c"removeFile\00"
@dryad_lit___dryad_main_13 = private unnamed_addr constant [12 x i8] c"/handle.txt\00"
@dryad_lit___dryad_main_14 = private unnamed_addr constant [6 x i8] c"write\00"
@dryad_lit___dryad_main_15 = private unnamed_addr constant [11 x i8] c"FileHandle\00"
@dryad_lit___dryad_main_16 = private unnamed_addr constant [15 x i8] c"Handle payload\00"
@dryad_lit___dryad_main_17 = private unnamed_addr constant [17 x i8] c"FileHandle_write\00"
@dryad_lit___dryad_main_18 = private unnamed_addr constant [17 x i8] c"FileHandle_close\00"
@dryad_lit___dryad_main_19 = private unnamed_addr constant [5 x i8] c"read\00"
@dryad_lit___dryad_main_20 = private unnamed_addr constant [16 x i8] c"FileHandle_read\00"
@dryad_lit___dryad_main_21 = private unnamed_addr constant [7 x i8] c"exists\00"

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  %slot2 = alloca i8*, align 8
  %slot3 = alloca i8*, align 8
  %slot4 = alloca i8*, align 8
  %slot5 = alloca i8*, align 8
  %slot6 = alloca i1, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v1 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v1, i8** %slot0, align 8
  %v2 = load i8*, i8** %slot0, align 8
  %v3 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  %v4 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @dryad_lit___dryad_main_3, i64 0, i64 0))
  %v5 = call i8* @dryad_intrinsic_slow(i16 65417, i8 3)
  %v6 = load i8*, i8** %slot0, align 8
  %v7 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  %v8 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  store i8* %v8, i8** %slot1, align 8
  %v9 = load i8*, i8** %slot0, align 8
  %v10 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  %v11 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  %v12 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  %v13 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v14 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_8, i64 0, i64 0))
  %v15 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([20 x i8], [20 x i8]* @dryad_lit___dryad_main_9, i64 0, i64 0))
  %v16 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  %v17 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_8, i64 0, i64 0))
  %v18 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v18, i8** %slot2, align 8
  %v19 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([9 x i8], [9 x i8]* @dryad_lit___dryad_main_8, i64 0, i64 0))
  %v20 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v21 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_13, i64 0, i64 0))
  %v22 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_14, i64 0, i64 0))
  %v23 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  store i8* %v23, i8** %slot3, align 8
  %v24 = load i8*, i8** %slot3, align 8
  %v25 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @dryad_lit___dryad_main_16, i64 0, i64 0))
  %v26 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  %v27 = load i8*, i8** %slot3, align 8
  %v28 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v29 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_13, i64 0, i64 0))
  %v30 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_19, i64 0, i64 0))
  %v31 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  store i8* %v31, i8** %slot4, align 8
  %v32 = load i8*, i8** %slot4, align 8
  %v33 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v33, i8** %slot5, align 8
  %v34 = load i8*, i8** %slot4, align 8
  %v35 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v36 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_13, i64 0, i64 0))
  %v37 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v38 = load i8*, i8** %slot1, align 8
  %v39 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @dryad_lit___dryad_main_3, i64 0, i64 0))
  %as_i8_0 = bitcast ptr %v38 to i8*
  %as_i8_1 = bitcast ptr %v39 to i8*
  %v40 = call i1 @dryad_variant_eq(i8* %as_i8_0, i8* %as_i8_1)
  br i1 %v40, label %bb1, label %bb2

bb1:
  %v41 = load i8*, i8** %slot2, align 8
  %v42 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([20 x i8], [20 x i8]* @dryad_lit___dryad_main_9, i64 0, i64 0))
  %as_i8_2 = bitcast ptr %v41 to i8*
  %as_i8_3 = bitcast ptr %v42 to i8*
  %v43 = call i1 @dryad_variant_eq(i8* %as_i8_2, i8* %as_i8_3)
  br label %bb2

bb2:
  %v44 = phi i1 [%v40, %bb0], [%v43, %bb1]
  br i1 %v44, label %bb3, label %bb4

bb3:
  %v45 = load i8*, i8** %slot5, align 8
  %v46 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @dryad_lit___dryad_main_16, i64 0, i64 0))
  %as_i8_4 = bitcast ptr %v45 to i8*
  %as_i8_5 = bitcast ptr %v46 to i8*
  %v47 = call i1 @dryad_variant_eq(i8* %as_i8_4, i8* %as_i8_5)
  br label %bb4

bb4:
  %v48 = phi i1 [%v44, %bb2], [%v47, %bb3]
  br i1 %v48, label %bb5, label %bb6

bb5:
  %v49 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_13, i64 0, i64 0))
  %v50 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %as_i8_6 = bitcast ptr %v50 to i8*
  %v51 = call i1 @dryad_unbox_bool(i8* %as_i8_6)
  %v52 = icmp eq i1 %v51, 0
  br label %bb6

bb6:
  %v53 = phi i1 [%v48, %bb4], [%v52, %bb5]
  store i1 %v53, i1* %slot6, align 8
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
