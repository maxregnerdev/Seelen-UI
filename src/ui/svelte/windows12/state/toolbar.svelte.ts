// Windows 12 Toolbar State
// Enables Windows 12 features on Windows 11

import { derived, writable } from "svelte/store";

// Toolbar visibility
interface ToolbarState {
  visible: boolean;
  position: "top" | "bottom" | "left" | "right";
  style: "floating" | "traditional" | "vertical";
  translucent: boolean;
  roundedCorners: boolean;
}

// Create toolbar state store
function createToolbarState() {
  const { subscribe, set, update } = writable<ToolbarState>({
    visible: true,
    position: "bottom",
    style: "floating",
    translucent: true,
    roundedCorners: true,
  });

  return {
    subscribe,
    // Show toolbar
    show: () => update((state) => ({ ...state, visible: true })),
    // Hide toolbar
    hide: () => update((state) => ({ ...state, visible: false })),
    // Toggle visibility
    toggle: () => update((state) => ({ ...state, visible: !state.visible })),
    // Set position
    setPosition: (position: ToolbarState["position"]) => update((state) => ({ ...state, position })),
    // Set style
    setStyle: (style: ToolbarState["style"]) => update((state) => ({ ...state, style })),
    // Toggle translucent
    toggleTranslucent: () => update((state) => ({ ...state, translucent: !state.translucent })),
    // Toggle rounded corners
    toggleRoundedCorners: () => update((state) => ({ ...state, roundedCorners: !state.roundedCorners })),
    // Reset to defaults
    reset: () =>
      set({
        visible: true,
        position: "bottom",
        style: "floating",
        translucent: true,
        roundedCorners: true,
      }),
  };
}

export const toolbarState = createToolbarState();

// Widgets state
interface WidgetsState {
  visible: boolean;
  position: "left" | "right" | "top" | "bottom";
  expanded: boolean;
  widgets: {
    weather: boolean;
    calendar: boolean;
    clock: boolean;
    system: boolean;
  };
}

function createWidgetsState() {
  const { subscribe, set, update } = writable<WidgetsState>({
    visible: true,
    position: "right",
    expanded: true,
    widgets: {
      weather: true,
      calendar: true,
      clock: true,
      system: true,
    },
  });

  return {
    subscribe,
    // Show widgets
    show: () => update((state) => ({ ...state, visible: true })),
    // Hide widgets
    hide: () => update((state) => ({ ...state, visible: false })),
    // Toggle visibility
    toggle: () => update((state) => ({ ...state, visible: !state.visible })),
    // Set position
    setPosition: (position: WidgetsState["position"]) => update((state) => ({ ...state, position })),
    // Toggle expanded
    toggleExpanded: () => update((state) => ({ ...state, expanded: !state.expanded })),
    // Toggle widget visibility
    toggleWidget: (widget: keyof WidgetsState["widgets"]) =>
      update((state) => ({
        ...state,
        widgets: {
          ...state.widgets,
          [widget]: !state.widgets[widget],
        },
      })),
    // Reset to defaults
    reset: () =>
      set({
        visible: true,
        position: "right",
        expanded: true,
        widgets: {
          weather: true,
          calendar: true,
          clock: true,
          system: true,
        },
      }),
  };
}

export const widgetsState = createWidgetsState();

// Search state
interface SearchState {
  visible: boolean;
  query: string;
  npuEnabled: boolean;
  suggestions: string[];
}

function createSearchState() {
  const { subscribe, set, update } = writable<SearchState>({
    visible: true,
    query: "",
    npuEnabled: true,
    suggestions: [],
  });

  return {
    subscribe,
    // Show search
    show: () => update((state) => ({ ...state, visible: true })),
    // Hide search
    hide: () => update((state) => ({ ...state, visible: false })),
    // Set query
    setQuery: (query: string) => update((state) => ({ ...state, query })),
    // Toggle NPU
    toggleNpu: () => update((state) => ({ ...state, npuEnabled: !state.npuEnabled })),
    // Set suggestions
    setSuggestions: (suggestions: string[]) => update((state) => ({ ...state, suggestions })),
    // Reset to defaults
    reset: () =>
      set({
        visible: true,
        query: "",
        npuEnabled: true,
        suggestions: [],
      }),
  };
}

export const searchState = createSearchState();

// Windows 12 features state
interface Windows12FeaturesState {
  // Feature flags
  features: {
    deepAiIntegration: boolean;
    redesignedUi: boolean;
    corepcArchitecture: boolean;
    enhancedSecurity: boolean;
    performanceOptimization: boolean;
    advancedGaming: boolean;
  };
  // System info
  system: {
    isWindows12: boolean;
    isWindows11: boolean;
    build: number;
  };
}

function createWindows12FeaturesState() {
  // Keep the complete store so derived() can receive the store itself.
  const store = writable<Windows12FeaturesState>({
    features: {
      deepAiIntegration: true,
      redesignedUi: true,
      corepcArchitecture: true,
      enhancedSecurity: true,
      performanceOptimization: true,
      advancedGaming: true,
    },
    system: {
      isWindows12: false,
      isWindows11: true,
      build: 22000,
    },
  });

  const { set, update } = store;

  return {
    subscribe: store.subscribe,

    // Enable feature
    enableFeature: (feature: keyof Windows12FeaturesState["features"]) =>
      update((state) => ({
        ...state,
        features: {
          ...state.features,
          [feature]: true,
        },
      })),

    // Disable feature
    disableFeature: (feature: keyof Windows12FeaturesState["features"]) =>
      update((state) => ({
        ...state,
        features: {
          ...state.features,
          [feature]: false,
        },
      })),

    // Toggle feature
    toggleFeature: (feature: keyof Windows12FeaturesState["features"]) =>
      update((state) => ({
        ...state,
        features: {
          ...state.features,
          [feature]: !state.features[feature],
        },
      })),

    // Check if feature is enabled
    isFeatureEnabled: (feature: keyof Windows12FeaturesState["features"]) =>
      derived(store, ($state) => $state.features[feature]),

    // Check if Windows 12 features are enabled
    isEnabled: derived(
      store,
      ($state) => $state.system.isWindows11 || $state.system.isWindows12,
    ),

    // Reset to defaults
    reset: () =>
      set({
        features: {
          deepAiIntegration: true,
          redesignedUi: true,
          corepcArchitecture: true,
          enhancedSecurity: true,
          performanceOptimization: true,
          advancedGaming: true,
        },
        system: {
          isWindows12: false,
          isWindows11: true,
          build: 22000,
        },
      }),
  };
}

export const windows12Features = createWindows12FeaturesState();

// Derived states
export const isToolbarFloating = derived(
  toolbarState,
  ($state) => $state.style === "floating",
);

export const isToolbarTranslucent = derived(
  toolbarState,
  ($state) => $state.translucent,
);

export const areWidgetsExpanded = derived(
  widgetsState,
  ($state) => $state.expanded,
);

export const isSearchNpuEnabled = derived(
  searchState,
  ($state) => $state.npuEnabled,
);
