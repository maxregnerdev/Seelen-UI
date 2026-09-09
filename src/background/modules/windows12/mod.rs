//! Windows 12 Features Module
//!
//! Implements Windows 12 features that are also enabled on Windows 11:
//! - Deep AI Integration: NPU acceleration for NLP, background tasks, cognitive search
//! - Redesigned User Interface: Floating taskbar, translucent elements, relocated widgets
//! - CorePC Modular Architecture: Component isolation, modular updates, resource optimization
//! - Enhanced Security Standards: Zero-trust, TPM 2.0, AI threat detection
//! - Performance and Energy Optimization: ARM support, dynamic resource allocation
//! - Advanced Gaming Features: DirectStorage, Auto-HDR, Xbox integration

pub mod corepc;
pub mod gaming;
pub mod npu;
pub mod performance;
pub mod security;
pub mod ui;

pub use corepc::*;
pub use gaming::*;
pub use npu::*;
pub use performance::*;
pub use security::*;
pub use ui::*;

use crate::error::Result;
use std::sync::{Arc, Mutex, OnceLock};

/// Windows 12 feature flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Windows12Feature {
    /// Deep AI Integration: NPU-powered AI features
    DeepAiIntegration,
    /// Redesigned User Interface: Modern modular UI
    RedesignedUi,
    /// CorePC Modular Architecture: Modular system foundation
    CorepcArchitecture,
    /// Enhanced Security Standards: Zero-trust security model
    EnhancedSecurity,
    /// Performance and Energy Optimization: Smart power management
    PerformanceOptimization,
    /// Advanced Gaming Features: DirectStorage, Auto-HDR, Xbox
    AdvancedGaming,
}

impl Windows12Feature {
    pub fn name(&self) -> &'static str {
        match self {
            Windows12Feature::DeepAiIntegration => "Deep AI Integration",
            Windows12Feature::RedesignedUi => "Redesigned User Interface",
            Windows12Feature::CorepcArchitecture => "CorePC Modular Architecture",
            Windows12Feature::EnhancedSecurity => "Enhanced Security Standards",
            Windows12Feature::PerformanceOptimization => "Performance and Energy Optimization",
            Windows12Feature::AdvancedGaming => "Advanced Gaming Features",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Windows12Feature::DeepAiIntegration => {
                "Built from the ground up with advanced artificial intelligence, relying heavily on dedicated Neural Processing Units (NPUs) to power system-wide features like enhanced natural language processing, automated background tasks, and cognitive search capabilities"
            }
            Windows12Feature::RedesignedUi => {
                "Features a modern, modular look that often includes a floating taskbar with rounded corners, translucent elements, and system widgets or search bars relocated for a cleaner, more adaptive desktop layout"
            }
            Windows12Feature::CorepcArchitecture => {
                "Utilizes a modular system foundation that allows Microsoft to separate core operating system components from apps and drivers, enabling better performance, quicker updates, and optimized resource use across different device types"
            }
            Windows12Feature::EnhancedSecurity => {
                "Focuses on a zero-trust security model, featuring advanced hardware-backed protections, continuous real-time threat detection via AI, and strict requirements for hardware security features like TPM 2.0"
            }
            Windows12Feature::PerformanceOptimization => {
                "Includes smarter power management geared toward modern mobile processors and ARM architectures, dynamic resource allocation to boost multitasking efficiency, and faster processing for local and cloud workloads"
            }
            Windows12Feature::AdvancedGaming => {
                "Upgraded support for technologies like DirectStorage optimizations, auto-HDR improvements, and tighter integration with Xbox services and cloud gaming ecosystems"
            }
        }
    }

    /// Check if this feature is available on Windows 11
    pub fn available_on_windows_11(&self) -> bool {
        match self {
            // All Windows 12 features are enabled on Windows 11
            _ => true,
        }
    }
}

/// Windows 12 system information
#[derive(Debug, Clone)]
pub struct Windows12Info {
    pub is_windows12: bool,
    pub is_windows11: bool,
    pub build: u32,
    pub features: Vec<Windows12Feature>,
}

impl Windows12Info {
    pub fn new() -> Self {
        use crate::utils::{is_windows_11, is_windows_12, windows_version_num};

        let is_win12 = is_windows_12();
        let is_win11 = is_windows_11();
        let build = windows_version_num();

        // All features are available on Windows 11 and 12
        let features = vec![
            Windows12Feature::DeepAiIntegration,
            Windows12Feature::RedesignedUi,
            Windows12Feature::CorepcArchitecture,
            Windows12Feature::EnhancedSecurity,
            Windows12Feature::PerformanceOptimization,
            Windows12Feature::AdvancedGaming,
        ];

        Self {
            is_windows12: is_win12,
            is_windows11: is_win11,
            build,
            features,
        }
    }

    /// Check if Windows 12 features should be enabled
    pub fn features_enabled(&self) -> bool {
        self.is_windows12 || self.is_windows11
    }
}

/// Global Windows 12 state
static WINDOWS12_STATE: OnceLock<Mutex<Windows12State>> = OnceLock::new();

struct Windows12State {
    pub info: Windows12Info,
    pub npu: NpuAccelerator,
    pub corepc: CorepcManager,
    pub security: SecurityManager,
    pub performance: PerformanceManager,
    pub gaming: GamingManager,
    pub ui: UiManager,
}

impl Windows12State {
    fn new() -> Self {
        Self {
            info: Windows12Info::new(),
            npu: NpuAccelerator::new(),
            corepc: CorepcManager::new(),
            security: SecurityManager::new(),
            performance: PerformanceManager::new(),
            gaming: GamingManager::new(),
            ui: UiManager::new(),
        }
    }

    fn init(&mut self) -> Result<()> {
        self.npu.init()?;
        self.corepc.init()?;
        self.security.init()?;
        self.performance.init()?;
        self.gaming.init()?;
        self.ui.init()?;
        Ok(())
    }
}

/// Initialize Windows 12 integration
/// This will enable Windows 12 features on both Windows 11 and Windows 12
pub fn init() -> Result<()> {
    let state = WINDOWS12_STATE.get_or_init(|| Mutex::new(Windows12State::new()));

    let mut state_lock = state
        .lock()
        .map_err(|_| "Failed to lock Windows 12 state")?;
    state_lock.init()
}

/// Get Windows 12 state
pub fn get() -> Option<Windows12Info> {
    WINDOWS12_STATE
        .get()
        .and_then(|state| state.lock().ok().map(|s| s.info.clone()))
}

/// Check if Windows 12 features are enabled
pub fn is_windows12_features_enabled() -> bool {
    get().map_or(false, |info| info.features_enabled())
}

/// Check if running on Windows 12
pub fn is_windows12() -> bool {
    get().map_or(false, |info| info.is_windows12)
}

/// Check if running on Windows 11
pub fn is_windows11() -> bool {
    get().map_or(false, |info| info.is_windows11)
}

/// Get available Windows 12 features
pub fn get_features() -> Vec<Windows12Feature> {
    get().map_or(Vec::new(), |info| info.features.clone())
}

/// Get NPU accelerator
pub fn npu() -> Option<Arc<Mutex<NpuAccelerator>>> {
    WINDOWS12_STATE.get().and_then(|state| {
        state
            .lock()
            .ok()
            .map(|s| Arc::new(Mutex::new(s.npu.clone())))
    })
}

/// Get CorePC manager
pub fn corepc() -> Option<Arc<Mutex<CorepcManager>>> {
    WINDOWS12_STATE.get().and_then(|state| {
        state
            .lock()
            .ok()
            .map(|s| Arc::new(Mutex::new(s.corepc.clone())))
    })
}

/// Get Security manager
pub fn security() -> Option<Arc<Mutex<SecurityManager>>> {
    WINDOWS12_STATE.get().and_then(|state| {
        state
            .lock()
            .ok()
            .map(|s| Arc::new(Mutex::new(s.security.clone())))
    })
}

/// Get Performance manager
pub fn performance() -> Option<Arc<Mutex<PerformanceManager>>> {
    WINDOWS12_STATE.get().and_then(|state| {
        state
            .lock()
            .ok()
            .map(|s| Arc::new(Mutex::new(s.performance.clone())))
    })
}

/// Get Gaming manager
pub fn gaming() -> Option<Arc<Mutex<GamingManager>>> {
    WINDOWS12_STATE.get().and_then(|state| {
        state
            .lock()
            .ok()
            .map(|s| Arc::new(Mutex::new(s.gaming.clone())))
    })
}

/// Get UI manager
pub fn ui() -> Option<Arc<Mutex<UiManager>>> {
    WINDOWS12_STATE.get().and_then(|state| {
        state
            .lock()
            .ok()
            .map(|s| Arc::new(Mutex::new(s.ui.clone())))
    })
}
