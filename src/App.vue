<template>
  <div class="page-wrapper">
    <nav class="topbar" data-tauri-drag-region>
      <div class="topbar-container">
        <div class="topbar-logo">
          <span class="logo-icon">🫧</span>
          <span class="logo-text">Bubble Widgets</span>
        </div>
        <ul class="topbar-menu">
          <li><router-link to="/" class="menu-link">Dashboard</router-link></li>
          <li>
            <router-link to="/braindump" class="menu-link"
              >Brain Dump</router-link
            >
          </li>
          <li>
            <router-link to="/timer" class="menu-link">Timer</router-link>
          </li>
          <li><a href="#" class="menu-link">Contact</a></li>
        </ul>
      </div>
    </nav>
    <router-view></router-view>
  </div>
</template>
