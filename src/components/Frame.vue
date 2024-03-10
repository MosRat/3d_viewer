<script setup>

</script>

<template>
  <div class="common-layout">
    <el-container>
      <el-header>Header</el-header>
      <el-container>
        <el-aside width="200px">Aside</el-aside>
        <el-container>
          <el-main>Main</el-main>
          <el-footer>Footer</el-footer>
        </el-container>
      </el-container>
    </el-container>
  </div>
</template>

<style scoped>
.common-layout {
  box-sizing: border-box;
  height: 600px;
  width: 1000px;
  padding: 10px;
  margin: 100px;

}

.el-container {
  height: 100%;
}

.el-main, .el-header, .el-footer, .el-aside {
  font-size: 1rem;
  text-align: center;
  border: #9cc2fc 2px solid;
  background: #313336;
  border-radius: 2px;
}
</style>