﻿using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MatrixEngine.StateManagment {
    public interface Provider<Output> {

        internal Output data { get; set; }


        public Output Get();



    }

}
