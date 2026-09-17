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

@dryad_lit___dryad_main_0 = private unnamed_addr constant [4 x i8] c"add\00"
@dryad_lit___dryad_main_1 = private unnamed_addr constant [6 x i8] c"print\00"
@dryad_lit___dryad_main_2 = private unnamed_addr constant [4 x i8] c"sub\00"
@dryad_lit___dryad_main_3 = private unnamed_addr constant [4 x i8] c"mul\00"
@dryad_lit___dryad_main_4 = private unnamed_addr constant [4 x i8] c"div\00"
@dryad_lit___dryad_main_5 = private unnamed_addr constant [4 x i8] c"mod\00"
@dryad_lit___dryad_main_6 = private unnamed_addr constant [6 x i8] c"float\00"
@dryad_lit___dryad_main_7 = private unnamed_addr constant [14 x i8] c"Hello, Dryad!\00"
@dryad_lit___dryad_main_8 = private unnamed_addr constant [5 x i8] c"bool\00"
@dryad_lit___dryad_main_9 = private unnamed_addr constant [4 x i8] c"not\00"
@dryad_lit___dryad_main_10 = private unnamed_addr constant [8 x i8] c"compare\00"

define i8* @__dryad_main() {
  %slot0 = alloca i32, align 8
  %slot1 = alloca i32, align 8
  %slot2 = alloca double, align 8
  %slot3 = alloca i8*, align 8
  %slot4 = alloca i1, align 8
  br label %bb0

bb0:
  %v0 = add i32 0, 10
  store i32 %v0, i32* %slot0, align 8
  %v1 = add i32 0, 3
  store i32 %v1, i32* %slot1, align 8
  %v2 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_0, i64 0, i64 0))
  call void @dryad_print(i8* %v2)
  %v3 = inttoptr i64 0 to i8*
  %v4 = load i32, i32* %slot0, align 8
  %v5 = load i32, i32* %slot1, align 8
  %v6 = add i32 %v4, %v5
  %print_box7 = call i8* @dryad_box_i32(i32 %v6)
  call void @dryad_print(i8* %print_box7)
  %v7 = inttoptr i64 0 to i8*
  %v8 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_2, i64 0, i64 0))
  call void @dryad_print(i8* %v8)
  %v9 = inttoptr i64 0 to i8*
  %v10 = load i32, i32* %slot0, align 8
  %v11 = load i32, i32* %slot1, align 8
  %v12 = sub i32 %v10, %v11
  %print_box13 = call i8* @dryad_box_i32(i32 %v12)
  call void @dryad_print(i8* %print_box13)
  %v13 = inttoptr i64 0 to i8*
  %v14 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_3, i64 0, i64 0))
  call void @dryad_print(i8* %v14)
  %v15 = inttoptr i64 0 to i8*
  %v16 = load i32, i32* %slot0, align 8
  %v17 = load i32, i32* %slot1, align 8
  %v18 = mul i32 %v16, %v17
  %print_box19 = call i8* @dryad_box_i32(i32 %v18)
  call void @dryad_print(i8* %print_box19)
  %v19 = inttoptr i64 0 to i8*
  %v20 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_4, i64 0, i64 0))
  call void @dryad_print(i8* %v20)
  %v21 = inttoptr i64 0 to i8*
  %v22 = load i32, i32* %slot0, align 8
  %v23 = load i32, i32* %slot1, align 8
  %v24 = call i8* @dryad_intrinsic_slow(i16 65308, i8 2)
  call void @dryad_print(i8* %v24)
  %v25 = inttoptr i64 0 to i8*
  %v26 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_5, i64 0, i64 0))
  call void @dryad_print(i8* %v26)
  %v27 = inttoptr i64 0 to i8*
  %v28 = load i32, i32* %slot0, align 8
  %v29 = load i32, i32* %slot1, align 8
  %v30 = call i8* @dryad_intrinsic_slow(i16 65300, i8 2)
  call void @dryad_print(i8* %v30)
  %v31 = inttoptr i64 0 to i8*
  %v32 = fadd double 0.0, 3.14
  store double %v32, double* %slot2, align 8
  %v33 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([6 x i8], [6 x i8]* @dryad_lit___dryad_main_6, i64 0, i64 0))
  call void @dryad_print(i8* %v33)
  %v34 = inttoptr i64 0 to i8*
  %v35 = load double, double* %slot2, align 8
  %v36 = add i32 0, 2
  %v37 = call i8* @dryad_intrinsic_slow(i16 65298, i8 2)
  call void @dryad_print(i8* %v37)
  %v38 = inttoptr i64 0 to i8*
  %v39 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([14 x i8], [14 x i8]* @dryad_lit___dryad_main_7, i64 0, i64 0))
  store i8* %v39, i8** %slot3, align 8
  %v40 = load i8*, i8** %slot3, align 8
  call void @dryad_print(i8* %v40)
  %v41 = inttoptr i64 0 to i8*
  %v42 = add i1 0, 1
  store i1 %v42, i1* %slot4, align 8
  %v43 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([5 x i8], [5 x i8]* @dryad_lit___dryad_main_8, i64 0, i64 0))
  call void @dryad_print(i8* %v43)
  %v44 = inttoptr i64 0 to i8*
  %v45 = load i1, i1* %slot4, align 8
  %print_box46 = call i8* @dryad_box_bool(i1 %v45)
  call void @dryad_print(i8* %print_box46)
  %v46 = inttoptr i64 0 to i8*
  %v47 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @dryad_lit___dryad_main_9, i64 0, i64 0))
  call void @dryad_print(i8* %v47)
  %v48 = inttoptr i64 0 to i8*
  %v49 = load i1, i1* %slot4, align 8
  %v50 = icmp eq i1 %v49, 0
  %print_box51 = call i8* @dryad_box_bool(i1 %v50)
  call void @dryad_print(i8* %print_box51)
  %v51 = inttoptr i64 0 to i8*
  %v52 = call i8* @dryad_box_lit_cstring(i8* getelementptr inbounds ([8 x i8], [8 x i8]* @dryad_lit___dryad_main_10, i64 0, i64 0))
  call void @dryad_print(i8* %v52)
  %v53 = inttoptr i64 0 to i8*
  %v54 = load i32, i32* %slot0, align 8
  %v55 = load i32, i32* %slot1, align 8
  %v56 = icmp sgt i32 %v54, %v55
  %print_box57 = call i8* @dryad_box_bool(i1 %v56)
  call void @dryad_print(i8* %print_box57)
  %v57 = inttoptr i64 0 to i8*
  %v58 = load i32, i32* %slot0, align 8
  %v59 = add i32 0, 10
  %v60 = icmp eq i32 %v58, %v59
  %print_box61 = call i8* @dryad_box_bool(i1 %v60)
  call void @dryad_print(i8* %print_box61)
  %v61 = inttoptr i64 0 to i8*
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
