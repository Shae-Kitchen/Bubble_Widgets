<script setup>
import { onMounted, ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";

const router = useRouter();
const isMainWindow = ref(true);
const isTauri = ref(false);

const windowConfigs = {
  dashboard: {
    label: "dashboard",
    title: "Dashboard",
    url: "/dashboard",
    width: 900,
    height: 600,
    route: "/dashboard",
  },
  braindump: {
    label: "braindump",
    title: "Brain Dump",
    url: "/braindump",
    width: 900,
    height: 650,
    route: "/braindump",
  },
  timer: {
    label: "timer",
    title: "Timer",
    url: "/timer",
    width: 720,
    height: 620,
    route: "/timer",
  },
};

onMounted(async () => {
  console.log("App.vue mounted");

  // Detect if we're running in Tauri by trying to use the API
  try {
    const current = getCurrentWebviewWindow();
    console.log("Successfully got current webview window");
    console.log("Current window label:", current.label);

    isTauri.value = true;
    isMainWindow.value = current.label === "main";
    console.log("isTauri: true");
    console.log("isMainWindow set to:", isMainWindow.value);

    // If this is a non-main window, navigate to the correct route based on label
    if (!isMainWindow.value) {
      console.log(
        "Non-main window detected, navigating to route for label:",
        current.label,
      );

      // Find the route for this window label
      const config = Object.values(windowConfigs).find(
        (cfg) => cfg.label === current.label,
      );
      if (config && config.route) {
        console.log("Found config for", current.label, ":", config);
        console.log(
          "Available routes:",
          router.getRoutes().map((r) => r.path),
        );
        console.log("Attempting to navigate to route:", config.route);

        try {
          await router.push({ path: config.route });
          console.log("Successfully navigated to:", config.route);
          console.log(
            "Current route after navigation:",
            router.currentRoute.value.path,
          );
        } catch (navError) {
          console.error("Navigation error:", navError);
        }
      } else {
        console.warn("No config found for window label:", current.label);
      }
    }
  } catch (error) {
    console.log("Not in Tauri environment:", error);
    isTauri.value = false;
    isMainWindow.value = true;
  }
});

async function openWindow(key) {
  console.log("openWindow called with key:", key);

  if (!isTauri.value) {
    console.log("Not in Tauri environment, aborting");
    return;
  }

  const config = windowConfigs[key];
  if (!config) {
    console.warn(`Window config not found for key: ${key}`);
    return;
  }

  try {
    console.log(
      `[openWindow] Attempting to create window: ${config.label} with URL: ${config.url}`,
    );
    console.log("[openWindow] Invoke parameters:", {
      label: config.label,
      title: config.title,
      url: config.url,
      width: config.width,
      height: config.height,
    });

    console.log("[openWindow] Calling invoke now...");
    const result = await invoke("create_window", {
      label: config.label,
      title: config.title,
      url: config.url,
      width: config.width,
      height: config.height,
    });
    console.log("[openWindow] Invoke completed! Result:", result);
    console.log(`[openWindow] Window created successfully: ${config.label}`);
  } catch (error) {
    console.error(
      `[openWindow] ERROR Failed to create window '${config.label}':`,
      error,
    );
    console.error("[openWindow] Error details:", JSON.stringify(error));
  }
}
</script>

<template>
  <div v-if="isMainWindow" class="launcher">
    <!-- Main window launcher -->
    <nav class="topbar" data-tauri-drag-region>
      <div class="topbar-container">
        <ul class="topbar-menu">
          <li>
            <button
              type="button"
              class="menu-link"
              data-tauri-drag-region="false"
              @click="openWindow('dashboard')"
            >
              Dashboard
            </button>
          </li>
          <li>
            <button
              type="button"
              class="menu-link"
              data-tauri-drag-region="false"
              @click="openWindow('braindump')"
            >
              Brain Dump
            </button>
          </li>
          <li>
            <button
              type="button"
              class="menu-link"
              data-tauri-drag-region="false"
              @click="openWindow('timer')"
            >
              Timer
            </button>
          </li>
          <li>
            <a href="#" class="menu-link" data-tauri-drag-region="false"
              >Contact</a
            >
          </li>
        </ul>
      </div>
    </nav>
  </div>
  <main v-else class="window-content">
    <!-- Child window content -->
    <router-view v-slot="{ Component }">
      <component :is="Component" v-if="Component" />
      <div v-else style="padding: 20px; color: red">
        <p>
          No component loaded. Current route:
          {{ router.currentRoute.value.path }}
        </p>
      </div>
    </router-view>
  </main>
</template>
