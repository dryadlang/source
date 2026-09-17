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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [28 x i8] c"http://example.com:8080/api\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [14 x i8] c"Http_parseUrl\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [7 x i8] c"scheme\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [5 x i8] c"host\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [5 x i8] c"port\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [41 x i8] c"HTTP/1.1 200 OK\0AContent-Length: 2\0A\0AOK\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [19 x i8] c"HttpResponse_parse\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [7 x i8] c"status\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [5 x i8] c"body\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [18 x i8] c"HttpClient_create\00"
@dryad_lit___dryad_main_11 = private unnamed_addr constant [10 x i8] c"client ok\00"

define i8* @__dryad_main() {
  %slot0 = alloca i8*, align 8
  %slot1 = alloca i8*, align 8
  %slot2 = alloca i8*, align 8
  %slot3 = alloca i8*, align 8
  br label %bb0

bb0:
  %v0 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([28 x i8], [28 x i8]* @dryad_lit___dryad_main_0, i64 0, i64 0))
  %v1 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v1, i8** %slot0, align 8
  %v2 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  call void @dryad_print(i8* %v2)
  %v3 = inttoptr i64 0 to i8*
  %v4 = load i8*, i8** %slot0, align 8
  %v5 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  %v6 = call i8* @dryad_intrinsic_slow(i16 71, i8 2)
  call void @dryad_print(i8* %v6)
  %v7 = inttoptr i64 0 to i8*
  %v8 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  call void @dryad_print(i8* %v8)
  %v9 = inttoptr i64 0 to i8*
  %v10 = load i8*, i8** %slot0, align 8
  %v11 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  %v12 = call i8* @dryad_intrinsic_slow(i16 71, i8 2)
  call void @dryad_print(i8* %v12)
  %v13 = inttoptr i64 0 to i8*
  %v14 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_5, i64 0, i64 0))
  call void @dryad_print(i8* %v14)
  %v15 = inttoptr i64 0 to i8*
  %v16 = load i8*, i8** %slot0, align 8
  %v17 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_5, i64 0, i64 0))
  %v18 = call i8* @dryad_intrinsic_slow(i16 71, i8 2)
  call void @dryad_print(i8* %v18)
  %v19 = inttoptr i64 0 to i8*
  %v20 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([41 x i8], [41 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  store i8* %v20, i8** %slot1, align 8
  %v21 = load i8*, i8** %slot1, align 8
  %v22 = call i8* @dryad_intrinsic_slow(i16 65417, i8 1)
  store i8* %v22, i8** %slot2, align 8
  %v23 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([7 x i8], [7 x i8]* @dryad_lit___dryad_main_8, i64 0, i64 0))
  call void @dryad_print(i8* %v23)
  %v24 = inttoptr i64 0 to i8*
  %v25 = load i8*, i8** %slot2, align 8
  %v26 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v26)
  %v27 = inttoptr i64 0 to i8*
  %v28 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_9, i64 0, i64 0))
  call void @dryad_print(i8* %v28)
  %v29 = inttoptr i64 0 to i8*
  %v30 = load i8*, i8** %slot2, align 8
  %v31 = call i8* @dryad_intrinsic_slow(i16 65411, i8 1)
  call void @dryad_print(i8* %v31)
  %v32 = inttoptr i64 0 to i8*
  %v33 = call i8* @dryad_intrinsic_slow(i16 65417, i8 0)
  store i8* %v33, i8** %slot3, align 8
  %v34 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([10 x i8], [10 x i8]* @dryad_lit___dryad_main_11, i64 0, i64 0))
  call void @dryad_print(i8* %v34)
  %v35 = inttoptr i64 0 to i8*
  %v36 = load i8*, i8** %slot3, align 8
  %v37 = call i8* @dryad_intrinsic_slow(i16 65280, i8 0)
  %as_i8_0 = bitcast ptr %v36 to i8*
  %as_i8_1 = bitcast ptr %v37 to i8*
  %v38 = call i1 @dryad_variant_neq(i8* %as_i8_0, i8* %as_i8_1)
  %print_box39 = call i8* @dryad_box_bool(i1 %v38)
  call void @dryad_print(i8* %print_box39)
  %v39 = inttoptr i64 0 to i8*
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
