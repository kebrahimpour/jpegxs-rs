# Clean-Room Implementation Roadmap

## Objective
Replace all derivative code with clean-room implementations to achieve full commercial licensing rights.

## Resources for Clean Implementation
- **Primary**: ISO/IEC 21122-1:2019 (JPEG XS Part 1: Core coding system)
- **Secondary**: ISO/IEC 21122-2:2019 (JPEG XS Part 2: Profiles and buffer models)
- **Academic Papers**: Published research on wavelet transforms and entropy coding
- **NOT ALLOWED**: Reference implementation source code

## Implementation Phases

### ✅ Already Clean (Can Commercialize Now)
- [x] Project structure and build system
- [x] CLI interface (`jpegxs-cli`)
- [x] Basic I/O operations (`jpegxs-io`)
- [x] Bit-level I/O
- [x] YUV file handling
- [x] Basic quantization
- [x] Test framework

### 🔄 Phase 1: Core Algorithms (Weeks 1-4)

#### Week 1-2: DWT from Specification
- [ ] Study ISO/IEC 21122-1:2019 Section 7.3
- [ ] Implement 5/3 reversible filter from equations
- [ ] Implement 9/7 irreversible filter from equations
- [ ] Create comprehensive tests using known transforms
- [ ] Replace current `dwt.rs` with clean version

#### Week 3-4: Entropy Coding
- [ ] Study ISO/IEC 21122-1:2019 Section 8
- [ ] Implement VLC (Variable Length Coding) tables
- [ ] Implement significance coding
- [ ] Implement refinement coding
- [ ] Test against standard test vectors

### 🔄 Phase 2: Bitstream Structure (Weeks 5-6)

#### Week 5: Packet Structure
- [ ] Study ISO/IEC 21122-1:2019 Section 9
- [ ] Implement packet headers
- [ ] Implement slice structure
- [ ] Implement precinct structure

#### Week 6: Rate Control
- [ ] Study ISO/IEC 21122-1:2019 Section 10
- [ ] Implement budget allocation
- [ ] Implement quantization optimization
- [ ] Implement rate-distortion optimization

### 🔄 Phase 3: Profiles and Optimization (Weeks 7-8)

#### Week 7: Profile Support
- [ ] Study ISO/IEC 21122-2:2019
- [ ] Implement Main profile
- [ ] Implement High profile
- [ ] Implement Light profile

#### Week 8: Performance Optimization
- [ ] SIMD optimizations (original work)
- [ ] Parallel processing (original work)
- [ ] Memory optimization (original work)

## Development Guidelines

### For Each Component:

1. **Research Phase**
   ```
   - Read relevant ISO/IEC sections
   - Study mathematical definitions
   - Review academic papers
   - NO reference code viewing
   ```

2. **Design Phase**
   ```
   - Create algorithm flowchart
   - Define data structures
   - Design API interface
   - Document approach
   ```

3. **Implementation Phase**
   ```
   - Write code from scratch
   - Follow Rust best practices
   - Add comprehensive comments
   - Include attribution to spec sections
   ```

4. **Validation Phase**
   ```
   - Unit tests from spec examples
   - Integration tests
   - Compliance verification
   - Performance benchmarks
   ```

5. **Documentation Phase**
   ```
   - Update DERIVATIVE-TRACKING.md
   - Mark as 🟢 Original
   - Document compliance with spec
   - Update licensing status
   ```

## Parallel Track Development

### Track A: Evaluation/Research (Current)
- Continue using existing derivative code
- For testing and development only
- Cannot be commercialized
- Located in `crates/` directory

### Track B: Commercial (New)
- Clean-room implementations only
- Can be fully commercialized
- Located in `commercial/` directory (to be created)
- Gradual migration from Track A

## Success Metrics

- [ ] All core algorithms implemented from spec
- [ ] Zero derivative code in commercial track
- [ ] Passing ISO/IEC 21122 compliance tests
- [ ] Performance within 20% of reference
- [ ] Full documentation of clean-room process

## Legal Checkpoints

Before commercialization, verify:
1. No code copied from reference
2. All algorithms from public specifications
3. Documentation trail of clean-room process
4. Legal review of implementation
5. Clear separation of tracks

## Timeline Summary

- **Weeks 1-4**: Core algorithms (DWT, Entropy)
- **Weeks 5-6**: Bitstream structure
- **Weeks 7-8**: Profiles and optimization
- **Week 9**: Legal review and documentation
- **Week 10**: Commercial release preparation

Total: 10 weeks to full commercial implementation

## Next Immediate Steps

1. Set up `commercial/` directory structure
2. Obtain ISO/IEC 21122 standards documents
3. Begin Week 1 DWT specification study
4. Create test vectors from spec examples
5. Start clean-room implementation log