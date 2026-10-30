#ifndef DRYAD_RUNTIME_GC_H
#define DRYAD_RUNTIME_GC_H

namespace dryad {

class GarbageCollector {
public:
    GarbageCollector();
    ~GarbageCollector();
    
    void collect();
    
private:
};

} // namespace dryad

#endif // DRYAD_RUNTIME_GC_H
