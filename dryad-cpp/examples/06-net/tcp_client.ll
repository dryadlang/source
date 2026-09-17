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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [10 x i8] c"127.0.0.1\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [18 x i8] c"TcpClient_connect\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [15 x i8] c"Connect failed\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [12 x i8] c"hello dryad\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [18 x i8] c"Buffer_FromString\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [5 x i8] c"send\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [8 x i8] c"receive\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [16 x i8] c"Buffer_AsString\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [15 x i8] c"Buffer_Dispose\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [6 x i8] c"close\00"

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @dryad_lit___dryad_main_0, i64 0, i64 0))
  %v1 = add i32 0, 9090
  %v2 = call i8* @dryad_intrinsic_slow(i16 65417, i8 2)
  store i8* %v2, i8** %slot0, align 8
  %v3 = load i8*, i8** %slot0, align 8
  %v4 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  %v5 = call i8* @dryad_intrinsic_slow(i16 65280, i8 0)
  %as_i8_0 = bitcast ptr %v4 to i8*
  %as_i8_1 = bitcast ptr %v5 to i8*
  %v6 = call i1 @dryad_variant_neq(i8* %as_i8_0, i8* %as_i8_1)
  br i1 %v6, label %bb2, label %bb1

bb1:
  %v7 = load i8*, i8** %slot0, align 8
  %v8 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  %as_i8_2 = bitcast ptr %v8 to i8*
  %v9 = call i1 @dryad_unbox_bool(i8* %as_i8_2)
  %v10 = icmp eq i1 %v9, 0
  br label %bb2

bb2:
  %v11 = phi i1 [%v6, %bb0], [%v10, %bb1]
  br i1 %v11, label %bb3, label %bb4

bb3:
  %v12 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([15 x i8], [15 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  call void @dryad_print(i8* %v12)
  %v13 = inttoptr i64 0 to i8*
  br label %bb5

bb4:
  %v14 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([12 x i8], [12 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  %v15 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v15, i8** %slot1, align 8
  %v16 = load i8*, i8** %slot0, align 8
  %v17 = load i8*, i8** %slot1, align 8
  %v18 = add i32 0, 0
  %v19 = add i32 0, 11
  %v20 = call i8* @dryad_intrinsic_slow(i16 65415, i8 3)
  %v21 = load i8*, i8** %slot0, align 8
  %v22 = load i8*, i8** %slot1, align 8
  %v23 = add i32 0, 0
  %v24 = add i32 0, 11
  %v25 = call i8* @dryad_intrinsic_slow(i16 65415, i8 3)
  %v26 = load i8*, i8** %slot1, align 8
  %v27 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  call void @dryad_print(i8* %v27)
  %v28 = inttoptr i64 0 to i8*
  %v29 = load i8*, i8** %slot1, align 8
  %v30 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  %v31 = load i8*, i8** %slot0, align 8
  %v32 = call i8* @dryad_intrinsic_slow(i16 65415, i8 0)
  br label %bb5

bb5:
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
