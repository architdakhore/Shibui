# FlowWM Development Roadmap

**Current Version:** 0.1.0 (Alpha Development)  
**Last Updated:** April 8, 2026

---

## 📍 Current Status

**Phase:** Phase 4 - Advanced Features & Polish  
**Progress:** 75% Complete  
**Focus:** Backend integration and testing preparation

---

## 🗺️ Roadmap Overview

```
Phase 1: Foundation          ✅ COMPLETE (100%)
Phase 2: Wayland Protocols   ✅ COMPLETE (95%)
Phase 3: Backend Integration ✅ COMPLETE (90%)
Phase 4: Advanced Features   🔄 IN PROGRESS (60%)
Phase 5: Testing & Release   ⏳ PENDING (40%)
```

---

## 📅 Release Timeline

### **Alpha 0.1** - Target: Week 10 (May 2026)

**Goal:** Functional compositor for early testing

**Features:**
- ✅ Core architecture complete
- ✅ All layout engines working
- ✅ Basic Wayland protocols
- ⏳ Winit backend functional
- ⏳ Basic window rendering
- ⏳ Can run in windowed mode for testing

**Status:** 80% complete

---

### **Alpha 0.2** - Target: Week 12 (May 2026)

**Goal:** Testable on real hardware

**Features:**
- ⏳ DRM backend working
- ⏳ Multi-monitor support (basic)
- ⏳ XWayland support (basic)
- ⏳ Window decorations
- ⏳ Basic animations
- ⏳ Configuration hot-reload

**Status:** 60% complete

---

### **Beta 0.5** - Target: Week 16 (June 2026)

**Goal:** Daily driver usable

**Features:**
- ⏳ Full XWayland support
- ⏳ Complete window decorations
- ⏳ Advanced animations
- ⏳ Blur and shadow effects
- ⏳ IPC and CLI tool (flowmsg)
- ⏳ Performance optimization
- ⏳ NVIDIA support (basic)

**Status:** 40% complete

---

### **Release Candidate 0.9** - Target: Week 20 (July 2026)

**Goal:** Production-ready

**Features:**
- ⏳ All features complete
- ⏳ Excellent performance (144+ FPS)
- ⏳ Comprehensive documentation
- ⏳ AUR package available
- ⏳ Active community support
- ⏳ Bug fixes from beta testing

**Status:** 20% complete

---

### **Stable 1.0** - Target: Week 24 (August 2026)

**Goal:** Hyprland-level maturity

**Features:**
- ⏳ Production stability
- ⏳ Full feature parity with Hyprland
- ⏳ Unique features polished
- ⏳ Strong community
- ⏳ Regular release cycle
- ⏳ Plugin system (v1)

**Status:** 0% complete

---

## 🎯 Feature Backlog

### **High Priority (Must Have)**

#### Backend & Rendering
- [ ] Complete DRM/KMS implementation
- [ ] Buffer management and page flipping
- [ ] VSync and adaptive sync
- [ ] Multi-GPU support
- [ ] HDR support (long-term)

#### Window Management
- [ ] XWayland integration
- [ ] Window decorations (borders, shadows)
- [ ] Title bar rendering
- [ ] Window rules and exceptions
- [ ] Floating window support

#### Input
- [ ] Touch gesture support
- [ ] Tablet support
- [ ] Advanced pointer gestures
- [ ] Input profiles per-device

### **Medium Priority (Should Have)**

#### Visual Effects
- [ ] Blur effects (Kawase blur)
- [ ] Shadow rendering
- [ ] Rounded corners
- [ ] Window animations
- [ ] Workspace switch animations

#### Configuration
- [ ] Config validation tool
- [ ] Config hot-reload
- [ ] Per-output configuration
- [ ] Per-app configuration
- [ ] Theme engine

#### System Integration
- [ ] Screen recording (PipeWire)
- [ ] Screenshot support
- [ ] Color management
- [ ] Night light feature
- [ ] Fractional scaling

### **Low Priority (Nice to Have)**

#### Advanced Features
- [ ] Plugin system
- [ ] Scripting API (Lua/Python)
- [ ] Remote desktop support
- [ ] Mobile/touch optimization
- [ ] VR/AR support (far future)

#### Polish
- [ ] System tray support
- [ ] Notifications daemon
- [ ] Settings GUI
- [ ] On-screen keyboard
- [ ] Screen lock

---

## 🔧 Technical Debt

### **Known Issues**

1. **Backend Abstraction**
   - Current: Skeleton implementations
   - Needed: Full production code
   - Priority: High

2. **Buffer Management**
   - Current: Placeholder code
   - Needed: Zero-copy buffers
   - Priority: High

3. **Error Handling**
   - Current: Basic anyhow errors
   - Needed: Comprehensive error types
   - Priority: Medium

4. **Testing Coverage**
   - Current: Minimal tests
   - Needed: 80%+ coverage
   - Priority: Medium

5. **Documentation**
   - Current: Good foundation
   - Needed: API docs, examples
   - Priority: Medium

---

## 📊 Sprint Planning

### **Sprint 1** (Weeks 7-8)
**Focus:** Complete Winit Backend

- [ ] Full Winit window integration
- [ ] Event loop implementation
- [ ] Basic rendering to window
- [ ] Input handling in winit
- [ ] Testing guide

**Deliverable:** Can run FlowWM in a window

---

### **Sprint 2** (Weeks 9-10)
**Focus:** Alpha 0.1 Release

- [ ] Bug fixes from winit testing
- [ ] Performance profiling
- [ ] Documentation polish
- [ ] Release announcement
- [ ] Community feedback collection

**Deliverable:** Alpha 0.1 release

---

### **Sprint 3** (Weeks 11-12)
**Focus:** DRM Backend

- [ ] Complete DRM initialization
- [ ] Buffer allocation
- [ ] Page flipping
- [ ] VSync implementation
- [ ] Multi-monitor basics

**Deliverable:** Can run on real hardware

---

### **Sprint 4** (Weeks 13-14)
**Focus:** XWayland Support

- [ ] XWayland integration
- [ ] X11 window management
- [ ] Clipboard support
- [ ] Selection handling
- [ ] Testing with X11 apps

**Deliverable:** X11 app compatibility

---

### **Sprint 5** (Weeks 15-16)
**Focus:** Beta Release

- [ ] Window decorations
- [ ] Advanced animations
- [ ] Performance optimization
- [ ] Bug fixes
- [ ] Beta testing program

**Deliverable:** Beta 0.5 release

---

## 📈 Success Metrics

### **Alpha Success**
- [ ] Can run in winit window
- [ ] Basic window management works
- [ ] All 4 layouts functional
- [ ] 10+ early testers
- [ ] 5+ bugs reported and fixed

### **Beta Success**
- [ ] Can run on real hardware
- [ ] XWayland apps work
- [ ] Daily driver usable
- [ ] 50+ beta testers
- [ ] 20+ bugs fixed
- [ ] Performance matches Hyprland

### **1.0 Success**
- [ ] Production stable
- [ ] All features complete
- [ ] 500+ users
- [ ] Active community
- [ ] Regular releases
- [ ] AUR package maintained

---

## 🤝 Community Goals

### **Q2 2026**
- [ ] GitHub organization
- [ ] Discord server
- [ ] Contribution guidelines
- [ ] First community PR
- [ ] First community theme

### **Q3 2026**
- [ ] 100+ GitHub stars
- [ ] 10+ contributors
- [ ] r/unixporn features
- [ ] First AUR package
- [ ] Wiki maintained by community

### **Q4 2026**
- [ ] 1000+ users
- [ ] Plugin ecosystem
- [ ] Regular release cycle
- [ ] Conference talk proposal
- [ ] Sponsorship program

---

## 🎓 Learning from Others

### **Hyprland's Success**
- ✅ Rapid feature development
- ✅ Strong community engagement
- ✅ Regular updates
- ✅ Good documentation
- ❌ Custom config language (hard to learn)
- ❌ C++ memory issues

**Our Approach:**
- ✅ TOML configuration (easier)
- ✅ Rust memory safety
- ✅ Monthly releases
- ✅ Comprehensive docs

### **niri's Approach**
- ✅ Focus on stability
- ✅ Clean architecture
- ✅ Modern Rust
- ❌ Slow development
- ❌ Limited features

**Our Approach:**
- ✅ Balanced speed and stability
- ✅ Full feature set from start
- ✅ Active development

### **MangoWM's Lessons**
- ✅ Simple and minimal
- ❌ Tag system confusing
- ❌ Limited documentation
- ❌ Small community

**Our Approach:**
- ✅ Traditional workspaces
- ✅ Extensive documentation
- ✅ Community-first development

---

## 📞 Feedback Loop

### **User Feedback Channels**
- GitHub Issues (bugs)
- GitHub Discussions (features)
- Discord (community)
- Reddit (r/unixporn)
- Twitter/X (announcements)

### **Feedback Integration**
1. Collect feedback weekly
2. Prioritize monthly
3. Implement in sprints
4. Release quarterly
5. Review annually

---

## 🎯 Long-Term Vision (2027+)

### **Version 2.0** (2027)
- Plugin ecosystem
- Advanced animations
- Mobile support
- Cloud sync

### **Version 3.0** (2028)
- AI-assisted window management
- Predictive layouts
- Cross-platform support
- Enterprise features

---

**This roadmap is living document.** Updated monthly based on progress and community feedback.

**Next Update:** May 2026

---

*Built with ❤️ for the Linux community*
