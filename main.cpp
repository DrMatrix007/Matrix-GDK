#include <iostream>
#include "src/engine/matrix_engine.h"

using namespace me;

class MyApplication : public Application
{
public:
    ecs::registry reg;
};

class ValueComponent : public ecs::component
{
public:
    int a = 0;

    virtual ~ValueComponent() {

    };

};

std::unique_ptr<Application> create_main_app()
{
    using namespace ecs;
    MyApplication *app = new MyApplication{};
    auto e = entity{};
    auto e1 = entity{};
    // auto v = me::ecs::component_vec{};
    //
    // v.set(T{}, e);
    auto &reg = app->reg;

    for (size_t i = 0; i < 20000; i++)
    {
        reg.set(entity{}, ValueComponent{});
    }
    locker<int> values = locker(0);

    auto wt1 = app->reg.write_components<ValueComponent>([&values](const entity &e, ValueComponent *p)
                                                         {
                                        auto [g,v] = values.write();
                                        p->a = v;
                                        v++; });

    auto t1 = app->reg.read<ValueComponent>([](const entity &e, const ValueComponent *p)
                                            {if (!(p->a %1000)){

                                            me::meout << "yoo: " << p->a << std::endl;
                                            } });

    auto t2 = app->reg.read<ValueComponent>([](const entity &e, const ValueComponent *p)
                                            { if (!(p->a %1000)){

                                            me::meout << "nice: " << p->a << std::endl;
                                            } });

    t2.join();
    t1.join();
    wt1.join();

    return std::unique_ptr<Application>(app);
}